/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-gpio-interrupts

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::{self as _};

static FLAG: AtomicBool = AtomicBool::new(false);

fn gpio_int_callback() {
    // Assert FLAG indicating a press button happened
    FLAG.store(true, Ordering::Relaxed);
}

fn main() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Take Peripherals
    let dp = Peripherals::take().unwrap();

    // Configure button pin as input
    let mut button = PinDriver::input(dp.pins.gpio0).unwrap();
    // Configure button pin with internal pull up
    button.set_pull(Pull::Up).unwrap();
    // Configure button pin to detect interrupts on a positive edge
    button.set_interrupt_type(InterruptType::PosEdge).unwrap();
    // Attach the ISR to the button interrupt
    unsafe { button.subscribe(gpio_int_callback).unwrap() }
    // Enable interrupts
    button.enable_interrupt().unwrap();

    // Set up a variable that keeps track of press button count
    let mut count = 0_u32;

    loop {
        // Check if global flag is asserted
        if FLAG.load(Ordering::Relaxed) {
            // Reset global flag
            FLAG.store(false, Ordering::Relaxed);
            // Update Press count and print
            count = count.wrapping_add(1);
            println!("Press Count {}", count);
        }
    }
}
