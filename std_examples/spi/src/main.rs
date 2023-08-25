/*
For a detailed explanation of this code check out the associated blog post:


GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://subscribepage.io/apollolabsnewsletter
*/

use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal::spi::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::config::Config;
use esp_idf_hal::spi::*;

fn main() -> ! {
    // Setup handler for device peripherals
    let peripherals = Peripherals::take().unwrap();

    // Create handles for SPI pins
    let sclk = peripherals.pins.gpio0;
    let mosi = peripherals.pins.gpio2;
    let cs = peripherals.pins.gpio3;

    // Instantiate SPI Driver
    let spi_drv = SpiDriver::new(
        peripherals.spi2,
        sclk,
        mosi,
        None::<gpio::AnyIOPin>,
        &SpiDriverConfig::new(),
    )
    .unwrap();

    // Configure Parameters for SPI device
    let config = Config::new().baudrate(2.MHz().into()).data_mode(Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    });

    // Instantiate SPI Device Driver and Pass Configuration
    let mut spi = SpiDeviceDriver::new(spi_drv, Some(cs), &config).unwrap();

    // Application

    // 1) Initalize Matrix Display

    // 1.a) Power Up Device

    // - Prepare Data to be Sent
    // 8-bit Data/Command Corresponding to Matrix Power Up
    let data: u8 = 0x01;
    // 4-bit Address of Shutdown Mode Command
    let addr: u8 = 0x0C;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    // Note that write method handles the CS pin state
    spi.write(&send_array).unwrap();

    // 1.b) Set up Decode Mode

    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to No Decode Mode
    let data: u8 = 0x00;
    // 4-bit Address of Decode Mode Command
    let addr: u8 = 0x09;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();

    // 1.c) Configure Scan Limit

    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to Scan Limit Displaying all digits
    let data: u8 = 0x07;
    // 4-bit Address of Scan Limit Command
    let addr: u8 = 0x0B;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();

    // 1.c) Configure Intensity

    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to (15/32 Duty Cycle) Medium Intensity
    let data: u8 = 0x07;
    // 4-bit Address of Intensity Control Command
    let addr: u8 = 0x0A;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();

    loop {
        let mut data: u8 = 1;
        // Iterate over all rows of LED matrix
        for addr in 1..9 {
            // addr refrences the row data will be sent to
            let send_array: [u8; 2] = [addr, data];
            // Shift a 1 with evey loop
            data = data << 1;

            // Send data just like earlier
            spi.write(&send_array).unwrap();

            // Delay for 500ms to show effect
            FreeRtos::delay_ms(500_u32);
        }

        // Clear the LED matrix row by row with 500ms delay in between
        for addr in 1..9 {
            let send_array: [u8; 2] = [addr, data];
            spi.write(&send_array).unwrap();
            FreeRtos::delay_ms(500_u32);
        }
    }
}
