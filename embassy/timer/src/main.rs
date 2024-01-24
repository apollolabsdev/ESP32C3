/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/embassy-on-esp-timers

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Instant;
use embedded_hal_async::digital::Wait;
use esp32c3_hal::gpio::{AnyPin, Input, PullUp};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;

#[embassy_executor::task]
async fn pulse1_timer(mut pin: AnyPin<Input<PullUp>>) {
    loop {
        // Wait for rising edge
        pin.wait_for_high().await.unwrap();
        // Capture time instant at rising edge
        let inst = Instant::now();
        // Wait for falling edge
        pin.wait_for_low().await.unwrap();
        // Calculate Duration
        let pwidth = Instant::checked_duration_since(&Instant::now(), inst).unwrap();
        // Print Duration
        esp_println::println!("Sq Wave 1 Pulse Width is {}ms", pwidth.as_millis());
        // Uncomment below line to reduce console print frequency
        // Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn pulse2_timer(mut pin: AnyPin<Input<PullUp>>) {
    loop {
        // Wait for rising edge
        pin.wait_for_high().await.unwrap();
        // Capture time instant at rising edge
        let inst = Instant::now();
        // Wait for falling edge
        pin.wait_for_low().await.unwrap();
        // Calculate Duration
        let pwidth = Instant::checked_duration_since(&Instant::now(), inst).unwrap();
        // Print Duration
        esp_println::println!("Sq Wave 1 Pulse Width is {}ms", pwidth.as_millis());
        // Uncomment below line to reduce console print frequency
        // Timer::after(Duration::from_millis(1000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initialize Embassy with needed timers
    let timer_group0 = esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group0.timer0);

    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");

    // Inititalize and configure pins
    // Acquire Handle to IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    // Configure pins as pull up input and degrade
    let pulse1 = io.pins.gpio0.into_pull_up_input().degrade();
    let pulse2 = io.pins.gpio1.into_pull_up_input().degrade();

    // Spawn pulse measurement tasks
    spawner.spawn(pulse1_timer(pulse1)).ok();
    spawner.spawn(pulse2_timer(pulse2)).ok();
}
