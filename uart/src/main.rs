#![no_std]
#![no_main]

use core::fmt::Write; // allows use to use the WriteLn! macro for easy printing
use debouncr::{debounce_16, Edge};
use esp32c3_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, uart::Uart, Rtc,
    IO,
};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    // Take Peripherals, Initialize Clocks, and Create a Handle for Each
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handles for the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
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
    let button = io.pins.gpio0.into_pull_up_input();

    // Initialize LED to on or off
    led.set_low().unwrap();

    // Create UART instance with default config
    let mut uart0 = Uart::new(peripherals.UART0);

    // Initialize debouncer to false because button is active low
    let mut debouncer = debounce_16(false);

    // Create and initialize a delay variable to manage delay loop
    let mut del_var = 10_0000_u32;

    // Variable to keep track of how many button presses occured
    let mut value: u8 = 0;

    // Application Loop
    loop {
        // Enter Delay Loop
        for _i in 1..del_var {
            // Keep checking if button got pressed
            if debouncer.update(button.is_low().unwrap()) == Some(Edge::Falling) {
                // If button is pressed print to derial and decrease the delay value
                writeln!(uart0, "Button Press {:02}\r", value).unwrap();
                // Increment value keeping track of button presses
                value = value.wrapping_add(1);
                // Decrement the amount of delay
                del_var = del_var - 2_5000_u32;
                // If updated delay value drops below threshold then reset it back to starting value
                if del_var < 2_5000_u32 {
                    del_var = 10_0000_u32;
                }
                // Exit delay loop since button was pressed
                break;
            }
        }
        led.toggle().unwrap();
    }
}
