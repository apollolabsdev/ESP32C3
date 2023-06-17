#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp32c3_hal::{
    clock::ClockControl,
    esp_riscv_rt::entry,
    gpio::{Event, Gpio0, Input, PullUp},
    interrupt,
    prelude::*,
    soc::peripherals::{Interrupt, Peripherals},
    timer::TimerGroup,
    Delay, Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;

// Global Variable Definitions
// Global variables are wrapped in safe abstractions.
// Peripherals are wrapped in a different manner than regular global mutable data.
// In the case of peripherals we must be sure only one refrence exists at a time.
// Refer to Chapter 6 of the Embedded Rust Book for more detail.

// Create a Global Variable for the GPIO Peripheral to pass around between threads.
static G_BUTTON: Mutex<RefCell<Option<Gpio0<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for the delay value to pass around between threads.
static G_DELAYMS: Mutex<Cell<u32>> = Mutex::new(Cell::new(2000_u32));

#[entry]
fn main() -> ! {
    // Take Peripherals, Initialize Clocks, and Create a Handle for Each
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handles for the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    // Disable the RTC and TIMG watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Instantiate and Create Handle for IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Instantiate and Create Handle for LED output & Button Input
    let mut led = io.pins.gpio4.into_push_pull_output();
    let mut button = io.pins.gpio0.into_pull_up_input();

    // Configure Button Pin for Interrupts
    // 1) Configure button for interrupt on falling edge and make it interrupt source
    button.listen(Event::FallingEdge);
    // 4) Enable gpio interrupts and set priority
    interrupt::enable(Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();

    // Enable Interrupts Globally in the risc-v Core
    unsafe {
        esp32c3_hal::esp_riscv_rt::riscv::interrupt::enable();
    }

    // Now that button is configured, move button into global context
    critical_section::with(|cs| G_BUTTON.borrow_ref_mut(cs).replace(button));

    // Create Delay Handle
    let mut delay = Delay::new(&clocks);

    // Application Loop
    loop {
        // Turn On LED
        led.set_high().unwrap();
        // Acquire updated G_DELAYMS and delay
        delay.delay_ms(critical_section::with(|cs| G_DELAYMS.borrow(cs).get()));
        // Turn off LED
        led.set_low().unwrap();
        // Acquire updated G_DELAYMS and delay
        delay.delay_ms(critical_section::with(|cs| G_DELAYMS.borrow(cs).get()));
        // delay.delay_ms(cortex_m::interrupt::free(|cs| G_DELAYMS.borrow(cs).get()));
    }

    #[interrupt]
    fn GPIO() {
        // Print for sanity to confirm interrupt is detecte
        println!("Button Press Interrupt!");
        // Start a Critical Section
        critical_section::with(|cs| {
            // Obtain Access to Delay Global Data and Adjust Delay
            G_DELAYMS
                .borrow(cs)
                .set(G_DELAYMS.borrow(cs).get() - 500_u32);
            if G_DELAYMS.borrow(cs).get() < 500_u32 {
                G_DELAYMS.borrow(cs).set(2000_u32);
            }
            // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
            G_BUTTON
                .borrow_ref_mut(cs)
                .as_mut()
                .unwrap()
                .clear_interrupt();
        });
    }
}
