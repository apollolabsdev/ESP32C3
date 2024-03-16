use esp32_nimble::{
    enums::{ConnMode, DiscMode},
    BLEAdvertisementData, BLEDevice,
};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys as _;

fn main() {
    esp_idf_svc::sys::link_patches();

    // Take ownership of device
    let ble_device = BLEDevice::take();
    // Obtain handle for advertiser
    let ble_advertiser = ble_device.get_advertising();

    // Specify Advertiser Name
    let mut ad_data = BLEAdvertisementData::new();
    ad_data.name("ESP32-C3 Peripheral");

    // Configure Advertiser with Specified Data
    ble_advertiser.lock().set_data(&mut ad_data).unwrap();

    // Make Advertiser Non-Connectable
    // Set Discovery Mode to General
    // Deactivate Scan Responses
    ble_advertiser
        .lock()
        .advertisement_type(ConnMode::Non)
        .disc_mode(DiscMode::Gen)
        .scan_response(false);

    // Start Advertising
    ble_advertiser.lock().start().unwrap();
    println!("Advertisement Started");
    loop {
        // Keep Advertising
        // Add delay to prevent watchdog from triggering
        FreeRtos::delay_ms(10);
    }
}
