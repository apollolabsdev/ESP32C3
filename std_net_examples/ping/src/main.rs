/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/esp32-embedded-rust-ping

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::ipv4::Ipv4Addr;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::ping::{Configuration as PingConfiguration, EspPing};
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    // Take Peripherals
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "Wokwi-GUEST".try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: "".try_into().unwrap(),
        channel: None,
    }))?;

    println!("Connecting to WiFi");

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    // This line is for Wokwi only so that the console output is formatted correctly
    println!("\x1b[20h");

    println!("Wifi Connected");

    println!("Pinging Google DNS (8.8.8.8)");

    let mut ping = EspPing::new(0_u32);

    let ping_res = ping.ping(
        Ipv4Addr::from_str("8.8.8.8").unwrap(),
        &PingConfiguration::default(),
    );

    match ping_res {
        Ok(summary) => println!(
            "Transmitted: {}, Recieved: {} Time: {:?}",
            summary.transmitted, summary.received, summary.time
        ),
        Err(e) => println!("{:?}", e),
    }

    loop {}
}
