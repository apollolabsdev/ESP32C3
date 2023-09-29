/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-connecting-wifi

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://subscribepage.io/apollolabsnewsletter
*/

use anyhow::{self, Error};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // Configure Wifi
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "Wokwi-GUEST".into(),
        password: "".into(),
        auth_method: AuthMethod::None,
        ..Default::default()
    }))?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Confirm Wifi Connection
    while !wifi.is_connected().unwrap() {
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }

    println!("Connected");

    Ok(())
}
