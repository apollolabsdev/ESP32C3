/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-timers

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://subscribepage.io/apollolabsnewsletter
*/

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::timer::config::Config;
use esp_idf_hal::timer::TimerDriver;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // Configure and Initialize Timer Drivers
    let config = Config::new();
    let mut timer1 = TimerDriver::new(peripherals.timer00, &config).unwrap();
    let mut timer2 = TimerDriver::new(peripherals.timer10, &config).unwrap();

    // Configure Pins that Will Read the Square Wave as Inputs
    let pin1 = PinDriver::input(peripherals.pins.gpio0).unwrap();
    let pin2 = PinDriver::input(peripherals.pins.gpio1).unwrap();

    // Declare and Init Variables that will Track Pin level
    let mut pin1_current_level: Level;
    let mut pin1_old_level: Level = Level::Low;

    let mut pin2_current_level: Level;
    let mut pin2_old_level: Level = Level::High;

    // Set Counter Start Value to Zero
    timer1.set_counter(0_u64).unwrap();
    timer2.set_counter(0_u64).unwrap();

    // Enable Counter
    timer1.enable(true).unwrap();
    timer2.enable(true).unwrap();

    // Declare and Init Variables that will Track Count Value
    let mut count1: u64 = 0;
    let mut count2: u64 = 0;

    loop {
        // Get Level of pin 1
        pin1_current_level = pin1.get_level();
        // // Get Level of pin 2
        pin2_current_level = pin2.get_level();

        // If pin 1 level changed from Low to High then reset count
        if (pin1_current_level != pin1_old_level) & (pin1_current_level == Level::High) {
            timer1.set_counter(0).unwrap();
            pin1_old_level = pin1_current_level;
        }

        // If pin 1 level changed from High to Low then capture count
        if (pin1_current_level != pin1_old_level) & (pin1_current_level == Level::Low) {
            count1 = timer1.counter().unwrap();
            pin1_old_level = pin1_current_level;
        }

        // If pin 2 level changed from Low to High then reset count
        if (pin2_current_level != pin2_old_level) & (pin2_current_level == Level::High) {
            timer2.set_counter(0).unwrap();
            pin2_old_level = pin2_current_level;
        }

        // If pin 2 level changed from High to Low then capture count
        if (pin2_current_level != pin2_old_level) & (pin2_current_level == Level::Low) {
            count2 = timer2.counter().unwrap();
            pin2_old_level = pin2_current_level;
        }

        // Calculate and Print Out the Pulse Width
        // Clock Frequency is 1 MHz According to Code
        println!("Sq Wave 1 Pulse Width is {:.1}ms", count1 / 1000);
        println!("Sq Wave 2 Pulse Width is {:.1}ms", count2 / 1000);
    }
}
