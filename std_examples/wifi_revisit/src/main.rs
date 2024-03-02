/*
For a detailed explanation of this code check out the associated blog posts:
https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-wifi-revisited

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use heapless::{String, Vec};
use std::fmt::Write;

fn main() -> anyhow::Result<()> {
    // Take Peripherals
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // Configure UART
    // Create handle for UART config struct
    let config = config::Config::default().baudrate(Hertz(115_200));

    // Instantiate UART
    let mut uart = UartDriver::new(
        peripherals.uart0,
        peripherals.pins.gpio21,
        peripherals.pins.gpio20,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    // This line is for Wokwi only so that the console output is formatted correctly
    uart.write_str("\x1b[20h").unwrap();

    uart.write_str("Enter Network SSID: ").unwrap();

    // Read and Buffer SSID
    let mut ssid = Vec::<u8, 32>::new();
    loop {
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK).unwrap();
        uart.write(&buf).unwrap();
        if buf[0] == 13 {
            break;
        }
        ssid.extend_from_slice(&buf).unwrap();
    }

    uart.write_str("\nEnter Network Password: ").unwrap();

    // Read and Buffer Password
    let mut password = Vec::<u8, 64>::new();
    loop {
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK).unwrap();
        uart.write(&[42]).unwrap();
        if buf[0] == 13 {
            break;
        }
        password.extend_from_slice(&buf).unwrap();
    }

    let ssid: String<32> = String::from_utf8(ssid).unwrap();
    let password: String<64> = String::from_utf8(password).unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid,
        bssid: None,
        auth_method: AuthMethod::None,
        password: password,
        channel: None,
    }))?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    println!("Wifi Connected");

    loop {}
}
