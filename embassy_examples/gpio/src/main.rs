/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/embassy-on-esp-gpio

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::Ordering;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal_async::digital::Wait;
use esp32c3_hal::gpio::{AnyPin, Input, PullUp};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;
use portable_atomic::AtomicU32;

// Global Variable to Control LED Rotation Speed
static BLINK_DELAY: AtomicU32 = AtomicU32::new(200_u32);

#[main]
async fn main(spawner: Spawner) {
    // Take Peripherals
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initilize Embassy Timers
    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    // Acquire Handle to IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    // Configure Delay Button to Pull Up input
    let del_but = io.pins.gpio2.into_pull_up_input().degrade();
    // Configure LED Array Pins to Output & Store in Array
    let mut leds = [
        io.pins.gpio1.into_push_pull_output().degrade(),
        io.pins.gpio10.into_push_pull_output().degrade(),
        io.pins.gpio19.into_push_pull_output().degrade(),
        io.pins.gpio18.into_push_pull_output().degrade(),
        io.pins.gpio4.into_push_pull_output().degrade(),
        io.pins.gpio5.into_push_pull_output().degrade(),
        io.pins.gpio6.into_push_pull_output().degrade(),
        io.pins.gpio7.into_push_pull_output().degrade(),
        io.pins.gpio8.into_push_pull_output().degrade(),
        io.pins.gpio9.into_push_pull_output().degrade(),
    ];
    // Enable GPIO Interrupts
    esp32c3_hal::interrupt::enable(
        esp32c3_hal::peripherals::Interrupt::GPIO,
        esp32c3_hal::interrupt::Priority::Priority1,
    )
    .unwrap();
    // Spawn Button Press Task
    spawner.spawn(press_button(del_but)).unwrap();

    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");

    // Enter Application Loop Blinking on LED at a Time
    loop {
        for led in &mut leds {
            led.set_high().unwrap();
            Timer::after(Duration::from_millis(
                BLINK_DELAY.load(Ordering::Relaxed) as u64
            ))
            .await;
            led.set_low().unwrap();
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}

#[embassy_executor::task]
async fn press_button(mut button: AnyPin<Input<PullUp>>) {
    loop {
        // Wait for Button Press
        button.wait_for_rising_edge().await.unwrap();
        esp_println::println!("Button Pressed!");
        // Retrieve Delay Global Variable
        let del = BLINK_DELAY.load(Ordering::Relaxed);
        // Adjust Delay Accordingly
        if del <= 50_u32 {
            BLINK_DELAY.store(200_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now 200ms");
        } else {
            BLINK_DELAY.store(del - 50_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now {}ms", del - 50_u32);
        }
    }
}
