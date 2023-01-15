use serde::Deserialize;
use tokio_serial::{available_ports, SerialPortInfo, SerialPortType};
//use bincode::{deserialize, serialize};
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub identity: KnownSerialDevice,
}

#[derive(Deserialize)]
pub struct KnownSerialDevice {
    pub serial_number: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

/// Checks if the given serial device matches the given serial number, manufacturer, and product.
/// If any of the fields are None, they are ignored.
/// Returns true if the device matches, false otherwise.
/// # Arguments
/// * `device` - The serial device to check
/// * `target_info` - The serial number, manufacturer, and product to check against
pub fn port_match(port: &SerialPortType, target_info: &KnownSerialDevice) -> bool {
    match port {
        SerialPortType::UsbPort(info) => {
            info.serial_number == Some(target_info.serial_number.clone())
                && info.manufacturer == target_info.manufacturer
                && info.product == target_info.product
        }
        _ => false,
    }
}

/// Returns the info about the first serial device that matches the given serial number, manufacturer, and product.
/// If any of the fields are None, they are ignored.
/// Returns None if no device matches.
/// # Arguments
/// * `target_info` - The serial number, manufacturer, and product to check against
pub fn find_matching_port(target_info: &KnownSerialDevice) -> Option<SerialPortInfo> {
    let ports = available_ports().unwrap();
    for port in ports {
        if port_match(&port.port_type, &target_info) {
            return Some(port);
        }
    }
    None
}

/// Takes a TOML Value and returns a KnownSerialDevice.
/// The TOML Value must have the following fields:
/// * `serial_number` - The serial number of the device
/// * `manufacturer` - The manufacturer of the device
/// * `product` - The product of the device
/// # Arguments
/// * `config` - The TOML Value to parse
/// # Panics
/// Panics if the TOML Value does not have the required fields.
/// Panics if the TOML Value has the wrong type for any of the fields.
pub fn parse_serial_info(toml_str: &str) -> Option<KnownSerialDevice> {
    let config: Config = toml::from_str(toml_str).expect("Failed to parse serial info");
    let serial_info = config.identity;
    let serial_number = serial_info.serial_number.to_string();
    let manufacturer = serial_info.manufacturer.unwrap_or("".to_string());
    let product = serial_info.product.unwrap_or("".to_string());
    Some(KnownSerialDevice {
        serial_number,
        manufacturer: Some(manufacturer),
        product: Some(product),
    })
}

/// Parses info about a serial device from a TOML file.
/// # Arguments
/// * `path` - The path to the TOML file
/// # Panics
/// Panics if the file cannot be read or parsed.
/// Panics if the file does not contain a serial_number, manufacturer, and product or is  not a
/// string, or is an empty string, or is not a valid UTF-8 string, or is not a valid TOML string.
pub fn parse_serial_info_from_file(path: &str) -> Option<KnownSerialDevice> {
    let toml = std::fs::read_to_string(path).expect("Failed to read serial info file");
    parse_serial_info(&toml)
}

// Tests
// -----
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_serial::UsbPortInfo;

    #[test]
    fn test_is_orchestrator_port() {
        let info = UsbPortInfo {
            serial_number: Some("0671FF555349898667170939".to_string()),
            manufacturer: Some("STMicroelectronics".to_string()),
            product: Some("STM32 STLink".to_string()),
            vid: 0,
            pid: 0,
        };

        let known_device = KnownSerialDevice {
            serial_number: "0671FF555349898667170939".to_string(),
            manufacturer: Some("STMicroelectronics".to_string()),
            product: Some("STM32 STLink".to_string()),
        };

        assert!(port_match(&SerialPortType::UsbPort(info), &known_device));
    }

    #[test]
    fn test_parse_serial_info() {
        let toml = r#"
        [identity]
        serial_number = "0671FF555349898667170939"
        manufacturer = "STMicroelectronics"
        product = "STM32 STLink"
        "#;

        let info = parse_serial_info(toml).unwrap();
        assert_eq!(info.serial_number, "0671FF555349898667170939".to_string());
        assert_eq!(info.manufacturer, Some("STMicroelectronics".to_string()));
        assert_eq!(info.product, Some("STM32 STLink".to_string()));
    }

    #[test]
    fn test_parse_serial_info_from_file() {
        let path = "sample.toml";
        let info = parse_serial_info_from_file(path).unwrap();
        assert_eq!(info.serial_number, "0671FF555349898667170939".to_string());
        assert_eq!(info.manufacturer, Some("STMicroelectronics".to_string()));
        assert_eq!(info.product, Some("STM32 STLink".to_string()));
    }
}
