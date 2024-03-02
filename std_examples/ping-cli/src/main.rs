/*
For a detailed explanation of this code check out the associated blog posts:
https://apollolabsblog.hashnode.dev/esp32-embedded-rust-ping-cli-app-part-1
https://apollolabsblog.hashnode.dev/esp32-embedded-rust-ping-cli-app-part-2

GitHub Repo containing source code and other examples:
https://github.com/apollolabsdev

For notifications on similar examples and more, subscribe to newsletter here:
https://www.theembeddedrustacean.com/subscribe
*/

use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::ping::{Configuration as PingConfiguration, EspPing, Summary};
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use menu::*;
use std::fmt::Write;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::time::Duration;

// CLI Root Menu Struct Initialization
const ROOT_MENU: Menu<UartDriver> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: hello_name,
                parameters: &[Parameter::Mandatory {
                    parameter_name: "name",
                    help: Some("Enter your name"),
                }],
            },
            command: "hw",
            help: Some("This is the help for the hello, name hw command!"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: ping_app,
                parameters: &[Parameter::Mandatory {
                    parameter_name: "hostname/IP",
                    help: Some("IP address or hostname"),
                },
                Parameter::NamedValue {
                    parameter_name: "count",
                    argument_name: "cnt",
                    help: Some("Packet count"),
                },
                Parameter::NamedValue {
                    parameter_name: "interval",
                    argument_name: "int",
                    help: Some("Interval between counts"),
                },
                Parameter::NamedValue {
                    parameter_name: "timeout",
                    argument_name: "to",
                    help: Some("timeout for each ping attempt"),
                },
                Parameter::NamedValue {
                    parameter_name: "size",
                    argument_name: "sz",
                    help: Some("Set the size of the packet"),
                },],
            },
            command: "ping",
            help: Some("
            Ping is a utility that sends ICMP Echo Request packets to a specified network host 
            (either identified by its IP address or hostname) to test connectivity and measure round-trip time.

            Usage: ping [options] <hostname/IP>
            
            Options:
              --count=<number>     Number of ICMP Echo Request packets to send (default is 4).
              --interval=<seconds> Set the interval between successive ping packets in seconds.
              --timeout=<seconds>  Specify a timeout value for each ping attempt.
              --size=<bytes>       Set the size of the ICMP packets.
              --help               Display this help message and exit.
            
            Examples:
              ping 192.168.1.1          # Ping the IP address 192.168.1.1
              ping example.com          # Ping the hostname 'example.com'
              ping -count=10 google.com     # Send 10 ping requests to google.com
              ping -interval=0.5 -size=100 example.com  # Ping with interval of 0.5 seconds and packet size of 100 bytes to 'example.com'
            "),
        },
    ],
    entry: None,
    exit: None,
};

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

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    println!("Wifi Connected");

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

// Callback function for hw command
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

// Callback function for ping command
fn ping_app<'a>(
    _menu: &Menu<UartDriver>,
    item: &Item<UartDriver>,
    args: &[&str],
    context: &mut UartDriver,
) {
    // Retreieve CLI Input
    let ip_str = argument_finder(item, args, "hostname/IP").unwrap().unwrap();

    // Resolve IP Address
    let addresses = (ip_str, 0)
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .next()
        .unwrap();

    // Convert to IP v4 type address
    let addr = match addresses {
        std::net::SocketAddr::V4(a) => *a.ip(),
        std::net::SocketAddr::V6(_) => {
            writeln!(context, "Address not compatible, try again").unwrap();
            return;
        }
    };

    // Create EspPing instance
    let mut ping = EspPing::new(0_u32);

    // Setup Default Ping Config
    let mut ping_config = PingConfiguration::default();

    // Obtain CLI Options and Modify Default Configuration Accordingly
    ping_config.count = 1;
    let mut ping_attempts = 4;

    match argument_finder(item, args, "count") {
        Ok(arg) => match arg {
            Some(cnt) => ping_attempts = FromStr::from_str(cnt).unwrap(),
            None => (),
        },
        Err(_) => (),
    }

    match argument_finder(item, args, "interval") {
        Ok(arg) => match arg {
            Some(inter) => {
                ping_config.interval = Duration::from_secs(FromStr::from_str(inter).unwrap())
            }
            None => (),
        },
        Err(_) => (),
    }

    match argument_finder(item, args, "timeout") {
        Ok(arg) => match arg {
            Some(to) => ping_config.timeout = Duration::from_secs(FromStr::from_str(to).unwrap()),
            None => (),
        },
        Err(_) => (),
    }

    match argument_finder(item, args, "size") {
        Ok(arg) => match arg {
            Some(sz) => ping_config.data_size = FromStr::from_str(sz).unwrap(),
            None => (),
        },
        Err(_) => (),
    }

    println!("{:?}", ping_config);

    // Update CLI
    // Pinging {IP} with {x} bytes of data
    writeln!(
        context,
        "Pinging {} [{:?}] with {} bytes of data\n",
        ip_str, addr, ping_config.data_size
    )
    .unwrap();

    let mut summary = Summary::default();
    let mut times: Vec<u128> = Vec::new();
    let mut rx_count = 0;

    // Ping 4 times and print results in following format:
    // Reply from {IP}: bytes={summary.recieved} time={summary.time} TTL={summary.timeout}
    for _n in 1..=ping_attempts {
        summary = ping.ping(addr, &ping_config).unwrap();

        writeln!(
            context,
            "Reply from {:?}: bytes = {}, time = {:?}, TTL = {:?}",
            addr, ping_config.data_size, summary.time, ping_config.timeout
        )
        .unwrap();

        // Update values for statistics
        times.push(summary.time.as_millis());
        if summary.transmitted == summary.received {
            rx_count += 1;
        }
    }

    // Print ping statstics in following format:
    // Ping statistics for {IP}:
    //      Packets: Sent = {sent}, Recieved  = {rec}, Lost = {loss} <{per}% Loss>
    // Approximate round trip times in milliseconds:
    //      Minimum = {min}ms, Maximum = {max}ms, Average = {avg}ms
    writeln!(context, "\nPing Statistics for {:?}", addr).unwrap();
    writeln!(
        context,
        "     Packets: Sent = {}, Recieved  = {}, Lost = {} <{}% loss>",
        ping_attempts,
        rx_count,
        ping_attempts - rx_count,
        ((ping_attempts - rx_count) / ping_attempts) * 100
    )
    .unwrap();
    writeln!(context, "Approximate round trip times in milliseconds:").unwrap();
    writeln!(
        context,
        "     Minimum = {} ms, Maximum = {} ms, Average = {} ms",
        times.iter().min().unwrap(),
        times.iter().max().unwrap(),
        times.iter().sum::<u128>() / times.len() as u128,
    )
    .unwrap();
}
