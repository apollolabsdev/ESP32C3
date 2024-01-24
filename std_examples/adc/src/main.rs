/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-analog-temperature-sensing-using-the-adc

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://subscribepage.io/apollolabsnewsletter
*/

use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::adc::attenuation::adc_atten_t_ADC_ATTEN_DB_11;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::adc::*;
use esp_idf_hal::gpio::Gpio4;
use esp_idf_hal::peripherals::Peripherals;
use libm::log;

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();

    // Configure ADC Driver
    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new()).unwrap();

    // Configure ADC Channel
    let mut adc_pin: esp_idf_hal::adc::AdcChannelDriver<
        '_,
        { adc_atten_t_ADC_ATTEN_DB_11 },
        Gpio4,
    > = AdcChannelDriver::new(peripherals.pins.gpio4).unwrap();

    const B: f64 = 3950.0; // B value of the thermistor
    const VMAX: f64 = 2500.0; // Full Range Voltage

    // Algorithm
    // 1) Get adc reading
    // 2) Convert to temperature
    // 3) Send over Serial
    // 4) Go Back to step 1

    loop {
        // Get ADC Reading
        let sample: u16 = adc.read(&mut adc_pin).unwrap();

        //Convert to temperature
        let temperature = 1. / (log(1. / (VMAX / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;

        // Print the temperature output
        println!("Temperature {:02} Celcius\r", temperature);
    }
}
