/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/embassy-on-esp-i2c-scanner

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp32c3_hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Obtain handle for GPIO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Initialize and configure I2C0
    let mut i2c0 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100u32.kHz(),
        &clocks,
    );

    // This line is for Wokwi only so that the console output is formatted correctly
    // esp_println::print!("\x1b[20h");

    // Start Scan at Address 1 going up to 127
    for addr in 1..=127 {
        println!("Scanning Address {}", addr as u8);

        // Scan Address
        let res = i2c0.read(addr as u8, &mut [0]);

        // Check and Print Result
        match res {
            Ok(_) => println!("Device Found at Address {}", addr as u8),
            Err(_) => println!("No Device Found"),
        }
    }

    // Loop Forever
    loop {}
}
