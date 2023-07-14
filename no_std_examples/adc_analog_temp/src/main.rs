#![no_std]
#![no_main]

use esp32c3_hal::{
    adc::{AdcConfig, Attenuation, ADC},
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;
use libm::log;

#[entry]
fn main() -> ! {
    // Take Peripherals, Initialize Clocks, and Create a Handle for Each
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
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

    // Create ADC Instance
    // Create handle for ADC configuration parameters
    let mut adc_config = AdcConfig::new();
    // Configure ADC pin
    let mut adc_pin =
        adc_config.enable_pin(io.pins.gpio1.into_analog(), Attenuation::Attenuation0dB);
    // Promote ADC peripheral to HAL-level Struct
    let analog = peripherals.APB_SARADC.split();
    // Create handle for ADC, configuring clock, and passing configuration handle
    let mut adc = ADC::adc(
        &mut system.peripheral_clock_control,
        analog.adc1,
        adc_config,
    )
    .unwrap();

    const B: f64 = 3950.0; // B value of the thermistor

    // Algorithm
    // 1) Get adc reading
    // 2) Convert to temperature
    // 3) Print to Console
    // 4) Go Back to step 1

    // Application
    loop {
        // Get ADC reading
        let sample: u16 = adc.read(&mut adc_pin).unwrap();
        // For blocking read
        // let sample: u16 = nb::block!(adc.read(&mut adc_pin)).unwrap();

        //Convert to temperature
        let temperature = 1. / (log(1. / (4096. / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;

        // Print the temperature output
        println!("Temperature {:02} Celcius\r", temperature);
    }
}
