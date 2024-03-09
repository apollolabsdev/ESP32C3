/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-http-server

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use anyhow;
use embedded_svc::http::Method;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    // Configure Wifi
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

    println!("Wifi Connected, Starting HTTP Server");

    // HTTP Configuration
    // Create HTTP Server Connection Handle
    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;

    // Define Server Request Handler Behaviour on Get for Root URL
    httpserver.fn_handler("/", Method::Get, |request| {
        // Retrieve html String
        let html = index_html();
        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(html.as_bytes())?;
        Ok(())
    })?;

    // Loop to Avoid Program Termination
    loop {
        sleep(Duration::from_millis(1000));
    }
}

fn index_html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
    Hello World from ESP!
    </body>
</html>
"#
    )
}
