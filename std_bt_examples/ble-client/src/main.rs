use esp32_nimble::{uuid128, BLEClient, BLEDevice};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::task::block_on;
use esp_idf_sys as _;

const DEVICE_NAME: &str = "iPhone";

fn main() {
    esp_idf_sys::link_patches();

    block_on(async {
        // Acquire BLE Device Handle
        let ble_device = BLEDevice::take();
        // Acquire Scan Handle
        let ble_scan = ble_device.get_scan();
        // Scan and Find Device/Server
        let device = ble_scan
            .active_scan(true)
            .interval(100)
            .window(99)
            .find_device(10000, |device| device.name() == DEVICE_NAME)
            .await
            .unwrap();

        if let Some(device) = device {
            // Create Client Handle
            let mut client = BLEClient::new();

            // Define Connect Behaviour
            client.on_connect(|client| {
                // Update Connect Parameters on Connect
                client.update_conn_params(120, 250, 0, 60).unwrap();
                println!("Connected to {}", DEVICE_NAME);
            });

            // Connect to advertising device using its address
            client.connect(device.addr()).await.unwrap();

            // Seek and Create Handle for the BLE Remote Server Service corresponding to the UUID
            let service = client
                .get_service(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1"))
                .await
                .unwrap();

            // Seek and Create Handle for the BLE Remote Server Characteristic corresponding to the UUID
            let uuid = uuid128!("681285a6-247f-48c6-80ad-68c3dce18585");
            let characteristic = service.get_characteristic(uuid).await.unwrap();

            loop {
                // Read & Print Characteristic Value
                let value = characteristic.read_value().await.unwrap();
                println!("Read Value: {:?}", value);

                // Wait 1 second before reading again
                FreeRtos::delay_ms(1000);
            }
        }
    });
}
