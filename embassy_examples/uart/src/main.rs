/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/embassy-on-esp-uart-transmitter

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embedded_hal_async::digital::Wait;
use esp32c3_hal::{
    clock::ClockControl,
    embassy, interrupt,
    peripherals::{Interrupt, Peripherals, UART0},
    prelude::*,
    Uart, UartTx, IO,
};
use esp_backtrace as _;

static MYSIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();

#[embassy_executor::task]
async fn uart_writer(mut tx: UartTx<'static, UART0>) {
    embedded_io_async::Write::write(
        &mut tx,
        b"UART Task Spawned. Waiting for Button Press...\r\n",
    )
    .await
    .unwrap();
    loop {
        let press_count = MYSIGNAL.wait().await;
        esp_println::println!("Button Pressed {} time(s)", press_count);
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initilize Embassy Timers
    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    // Configure UART
    let uart0 = Uart::new(peripherals.UART0, &clocks);
    let (tx, _) = uart0.split();

    // Configure GPIO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut button = io.pins.gpio2.into_pull_up_input();

    // Enable Interrupts for GPIO
    interrupt::enable(Interrupt::GPIO, interrupt::Priority::Priority1).unwrap();

    spawner.spawn(uart_writer(tx)).ok();

    let mut press_count = 0;
    loop {
        // Detect and Count Button Presses
        button.wait_for_rising_edge().await.unwrap();
        press_count += 1;
        // Signal Press Count
        MYSIGNAL.signal(press_count);
    }
}
