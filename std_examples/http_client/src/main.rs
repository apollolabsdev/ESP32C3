/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-http-client

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://subscribepage.io/apollolabsnewsletter
*/

use anyhow;
use embedded_svc::http::client::Client;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::{Configuration as HttpConfig, EspHttpConnection};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};

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

    // Print Out Wifi Connection Configuration
    while !wifi.is_connected().unwrap() {
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }

    println!("Wifi Connected, Intiatlizing HTTP");

    // HTTP Configuration
    // Create HTTPS Connection Handle
    let httpconnection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    // Create HTTPS Client
    let mut httpclient = Client::wrap(httpconnection);

    // HTTP Request Submission
    // Define URL
    let url = "https://httpbin.org/get";

    // Prepare request
    let request = httpclient.get(url)?;

    // Log URL and type of request
    println!("-> GET {}", url);

    // Submit Request and Store Response
    let response = request.submit()?;

    // HTTP Response Processing
    let status = response.status();
    println!("<- {}", status);

    match response.header("Content-Length") {
        Some(data) => {
            println!("Content-Length: {}", data);
        }
        None => {
            println!("No Content-Length Header");
        }
    }
    match response.header("Date") {
        Some(data) => {
            println!("Date: {}", data);
        }
        None => {
            println!("No Date Header");
        }
    }

    Ok(())
}
