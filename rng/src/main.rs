#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl, esp_riscv_rt::entry, peripherals::Peripherals, prelude::*,
    timer::TimerGroup, Delay, Rng, Rtc,
};
use esp_backtrace as _;
use esp_println::{print, println};

static MOCK_SENTENCES: &'static str = include_str!("nmea.rs");

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

    let mut delay = Delay::new(&clocks);

    // Configure terminal: enable newline conversion
    // This is for no std Rust using Wokwi as the output gets shifted
    print!("\x1b[20h");

    let mut rng = Rng::new(peripherals.RNG);

    // Application Loop
    loop {
        let num = rng.random() as u8;
        let sentence = MOCK_SENTENCES.lines().nth(num as usize).unwrap();
        println!("{}", sentence);
        delay.delay_ms(2000_u32);
    }
}
