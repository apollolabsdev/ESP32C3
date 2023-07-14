#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO};
use esp_backtrace as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Take Peripherals, Initialize Clocks, and Create a Handle for Each
    let peripherals = Peripherals::take().unwrap();
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

    // Create and initialize a delay variable to manage delay loop
    let mut del_var = 10_0000_u32;

    // Initialize LED to on or off
    led.set_low().unwrap();

    // Application Loop
    loop {
        for _i in 1..del_var {
            // Check if button got pressed
            if button.is_low().unwrap() {
                // If button pressed decrease the delay value
                del_var = del_var - 2_5000_u32;
                // If updated delay value reaches zero then reset it back to starting value
                if del_var < 2_5000 {
                    del_var = 10_0000_u32;
                }
            }
        }
        // Toggle LED
        led.toggle().unwrap();
    }
}
