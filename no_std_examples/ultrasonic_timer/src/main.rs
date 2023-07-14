#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, systimer::SystemTimer,
    timer::TimerGroup, Delay, Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;

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

    // Instantiate and Create Handle for trigger output & echo input
    let mut trig = io.pins.gpio1.into_push_pull_output();
    let echo = io.pins.gpio0.into_floating_input();
    //CHECK IF INTERNAL PULL UP IS OK

    let mut delay = Delay::new(&clocks);

    // Application Loop
    loop {
        // 1) Set pin ouput to low for 5 us to get clean low pulse
        trig.set_low().unwrap();
        delay.delay_us(5_u32);

        // 2) Set pin output to high (trigger) for 10us
        trig.set_high().unwrap();
        delay.delay_us(10_u32);
        trig.set_low().unwrap();

        // Wait until pin goes high
        while !echo.is_high().unwrap() {}

        // Kick off timer measurement
        let echo_start = SystemTimer::now();

        // Wait until pin goes low
        while !echo.is_low().unwrap() {}

        // Collect current timer count
        let echo_end = SystemTimer::now();

        // Calculate the elapsed timer
        let echo_dur = echo_end.wrapping_sub(echo_start);

        // Calculate the distance in cms using formula in datasheet
        let distance_cm = echo_dur / 16 / 58;

        // Print the distance output
        println!("Distance {} cm\r", distance_cm);
    }
}
