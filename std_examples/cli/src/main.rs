/*
For a detailed explanation of this code check out the associated blog post:
https://apollolabsblog.hashnode.dev/esp32-embedded-rust-simple-cli

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use menu::*;
use std::fmt::Write;

// CLI Root Menu Struct Initialization
const ROOT_MENU: Menu<UartDriver> = Menu {
    label: "root",
    items: &[&Item {
        item_type: ItemType::Callback {
            function: hello_name,
            parameters: &[Parameter::Mandatory {
                parameter_name: "name",
                help: Some("Enter your name"),
            }],
        },
        command: "hw",
        help: Some("This is an embedded CLI terminal. Check the summary for the list of supported commands"),
    }],
    entry: None,
    exit: None,
};

fn main() {
    // Take Peripherals
    let peripherals = Peripherals::take().unwrap();

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

    // This line is for Wokwi only so that the console output is formatted correctly
    uart.write_str("\x1b[20h").unwrap();

    // Create a buffer to store CLI input
    let mut clibuf = [0u8; 64];
    // Instantiate CLI runner with root menu, buffer, and uart
    let mut r = Runner::new(ROOT_MENU, &mut clibuf, uart);

    loop {
        // Create single element buffer for UART characters
        let mut buf = [0_u8; 1];
        // Read single byte from UART
        r.context.read(&mut buf, BLOCK).unwrap();
        // Pass read byte to CLI runner for processing
        r.input_byte(buf[0]);
    }
}

// Callback function for hw commans
fn hello_name<'a>(
    _menu: &Menu<UartDriver>,
    item: &Item<UartDriver>,
    args: &[&str],
    context: &mut UartDriver,
) {
    // Print to console passed "name" argument
    writeln!(
        context,
        "Hello, {}!",
        argument_finder(item, args, "name").unwrap().unwrap()
    )
    .unwrap();
}
