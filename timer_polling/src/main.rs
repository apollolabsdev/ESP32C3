#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO,
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

    // Instantiate and Create Handle for Timer
    let mut timer0 = timer_group0.timer0;

    // Initialize LED to on or off
    led.set_low().unwrap();

    // Create and initialize a delay variable to manage delay duration
    let mut del_var = 2000_u32.millis();

    // Application Loop
    loop {
        // Start counter with with del_var duration
        timer0.start(del_var);
        // Enter loop and check for button press until counter reaches del_var
        while timer0.wait() != Ok(()) {
            if button.is_low().unwrap() {
                // If button pressed decrease the delay value by 500 ms
                del_var = del_var - 1000_u32.millis();
                // If updated delay value drops below 500 ms then reset it back to starting value to 2 secs
                if del_var.to_millis() < 1000_u32 {
                    del_var = 2000_u32.millis();
                }
                // Exit delay loop since button was pressed
                break;
            }
        }
        // Toggle LED
        led.toggle().unwrap();
    }
}
