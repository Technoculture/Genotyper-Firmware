#![allow(unused_imports)]
#![allow(dead_code)]

use bincode::{deserialize, serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::{available_ports, new as new_serial, SerialPortType, UsbPortInfo};

enum Event {
    Read(Vec<u8>),
    Write(Vec<u8>),
}

const SERIAL_BAUD: u32 = 115_200;
const SERIAL_TIMEOUT: u64 = 100;
const ORCHESTRATOR_SERIAL: &str = "0671FF555349898667170939";
const ORCHESTRATOR_MANUFACTURER: &str = "STMicroelectronics";
const ORCHESTRATOR_PRODUCT: &str = "STM32 STLink";

/// Checks if the given port is the orchestrator port.
/// This is done by checking the USB port info.
/// To be an orchestrator port, the port must be a USB port, and the
/// serial number, manufacturer and product must match the expected values.
///
/// > Note: This is a hacky way to check if the port is the orchestrator port.
fn is_orchestrator_port(port: &SerialPortType) -> bool {
    match port {
        SerialPortType::UsbPort(info) => {
            info.serial_number == Some(ORCHESTRATOR_SERIAL.to_string())
                && info.manufacturer == Some(ORCHESTRATOR_MANUFACTURER.to_string())
                && info.product == Some(ORCHESTRATOR_PRODUCT.to_string())
        }
        _ => false,
    }
}

#[tokio::main]
async fn main() {
    let ports = available_ports().unwrap();
    //for port in ports.iter_mut() {
    //    println!("{}: {:?}", port.port_name, port.port_type);
    //}

    ports
        .into_iter()
        .filter(|port| is_orchestrator_port(&port.port_type))
        .for_each(|port| {
            println!("{}: {:?}", port.port_name, port.port_type);
        });

    //let port = new_serial("/dev/ttyUSB0", 115_200)
    //    .timeout(std::time::Duration::from_millis(100))
    //    .open()
    //    .expect("Failed to open port");
    //println!("Port opened: {:?}", port.name());
}
