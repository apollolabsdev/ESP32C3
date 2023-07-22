use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;

// Message to Send
const MESSAGE: &str = "Hello";
// Key Value (Can be any value from 1 to 255)
const KEY: u8 = 212;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio5;
    let rx = peripherals.pins.gpio6;

    let config = config::Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();

    let mut rec = Vec::new();

    // Print Message to be Sent
    println!("Sent Message in Text: {}", MESSAGE);
    println!("Sent Message in Values: {:?}", MESSAGE.as_bytes());

    // Garble Message
    let gmsg: Vec<u8> = MESSAGE.as_bytes().iter().map(|m| m ^ KEY).collect();

    // Print Garbled Message
    println!("Sent Garbled Message Values: {:?}", gmsg);

    // Send Garbled Message u8 Values One by One until Full Array is Sent
    for letter in gmsg.iter() {
        // Send Garbled Message Value
        uart.write(&[*letter]).unwrap();

        // Recieve Garbled Message Value
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK).unwrap();

        // Buffer Recieved Message Value
        rec.extend_from_slice(&buf);
    }

    // Print Recieved Garbled Message Values
    println!("Recieved Garbled Message Values: {:?}", rec);

    // UnGarble Message
    let ugmsg: Vec<u8> = rec.iter().map(|m| m ^ KEY).collect();
    println!("Ungarbled Message in Values: {:?}", ugmsg);

    // Print Recovered Message
    if let Ok(rmsg) = std::str::from_utf8(&ugmsg) {
        println!("Recieved Message in Text: {:?}", rmsg);
    };

    loop {}
}
