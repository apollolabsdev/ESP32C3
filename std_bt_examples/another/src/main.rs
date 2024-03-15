use esp32_nimble::BLEDevice;
use esp_idf_hal::task::block_on;
use esp_idf_sys as _;

fn main() {
    esp_idf_svc::sys::link_patches();

    block_on(async {
        let ble_device = BLEDevice::take();
        let ble_scan = ble_device.get_scan();
        ble_scan
            .active_scan(true)
            .interval(100)
            .window(50)
            .on_result(|_scan, param| {
                println!(
                    "Advertised Device Name: {:?}, Address: {:?} dB, RSSI: {:?}",
                    param.name(),
                    param.addr(),
                    param.rssi()
                );
            });
        ble_scan.start(5000).await.unwrap();
        println!("Scan finished");
    });
}
