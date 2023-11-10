/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-mqtt-subscriber

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use anyhow;
use embedded_svc::mqtt::client::Event;
use embedded_svc::mqtt::client::QoS;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "SSID".into(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: "PASSWORD".into(),
        channel: None,
    }))?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    // Print Out Wifi Connection Configuration
    while !wifi.is_connected().unwrap() {
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }

    println!("Wifi Connected");

    // Set up handle for MQTT Config
    let mqtt_config = MqttClientConfiguration::default();

    // Create Client Instance and Define Behaviour on Event
    let mut client = EspMqttClient::new(
        "mqtt://broker.mqttdashboard.com",
        &mqtt_config,
        move |message_event| {
            match message_event.as_ref().unwrap() {
                Event::Connected(_) => println!("Connected"),
                Event::Subscribed(id) => println!("Subscribed to {} id", id),
                Event::Received(msg) => {
                    if msg.data() != [] {
                        println!("Recieved {}", std::str::from_utf8(msg.data()).unwrap())
                    }
                }
                _ => println!("{:?}", message_event.as_ref().unwrap()),
            };
        },
    )?;

    // Subscribe to MQTT Topic
    client.subscribe("testtopic/1", QoS::AtLeastOnce)?;

    loop {
        // Keep waking up device to avoid watchdog reset
        sleep(Duration::from_millis(1000));
    }
}
