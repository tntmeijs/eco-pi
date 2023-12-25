mod obis;

use std::{fs, time::Duration};

use log::{error, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};

const SETTINGS_FILE_PATH: &str = "./settings.yaml";

#[derive(Debug, Deserialize, Serialize)]
struct Configuration {
    port_name: String,
    baud_rate: u32,
    connect_timeout_ms: u64,
}

impl Default for Configuration {
    /// Creates a configuration with sensible default values
    fn default() -> Self {
        Self {
            port_name: "/dev/ttyUSB0".to_owned(),
            baud_rate: 115_200,
            connect_timeout_ms: 20_000,
        }
    }
}

struct Transmission {
    identifier: String,
    messages: Vec<TransmissionMessage>,
}

struct TransmissionMessage {}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    if let Ok(data) = fs::read(SETTINGS_FILE_PATH) {
        match serde_yaml::from_slice::<Configuration>(&data) {
            Ok(configuration) => run(&configuration),
            Err(e) => error!("Unable to read YAML settings file: {}", e),
        }
    } else {
        warn!("No configuration file found - a new one will be created for you to modify (see \"./settings.yaml\")");
        warn!("Please re-run the application once you are happy with the configuration");

        match serde_yaml::to_string(&Configuration::default()) {
            Ok(yaml) => {
                if let Err(e) = fs::write(SETTINGS_FILE_PATH, yaml) {
                    error!("Failed to write default configuration file to disk: {}", e);
                }
            }
            Err(e) => error!("Failed to serialize settings to YAML string: {}", e),
        };
    }
}

fn run(configuration: &Configuration) {
    info!(
        "Attemping to open serialport \"{}\" with a baud rate of {}",
        configuration.port_name, configuration.baud_rate
    );

    match serialport::new(configuration.port_name.to_owned(), configuration.baud_rate)
        .timeout(Duration::from_millis(configuration.connect_timeout_ms))
        .open()
    {
        Ok(mut port) => listen(&mut port),
        Err(e) => {
            error!("Unable to open serial port: {}", e);

            if let Ok(ports) = serialport::available_ports() {
                info!(
                    "Available serial ports:\n{}",
                    ports
                        .iter()
                        .map(|p| format!("\t\t  \"{}\"", p.port_name))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        }
    }
}

fn listen(port: &mut Box<dyn serialport::SerialPort>) {
    let transmission_end_pattern =
        Regex::new(r"!\w{4}\r\n").expect("Failed to compile transmission start regex");

    // A transmission consists out of a data payload with a maximum of 1024 characters
    // Since there are only a handful of extra characters needed to make up a message, a buffer of 2048 bytes suffices
    let mut buffer: Vec<u8> = vec![0; 2048];
    let mut transmission = String::new();

    loop {
        match port.read(buffer.as_mut_slice()) {
            Ok(size) => {
                transmission.push_str(&String::from_utf8_lossy(&buffer[..size]));

                // End of the transmission
                if transmission_end_pattern.is_match(&transmission) {
                    process(&transmission);
                    transmission.clear();
                }
            }
            Err(e) => error!("Failed to read serial port data: {}", e),
        }
    }
}

fn process(transmission: &str) -> Option<Transmission> {
    // Header and data is split by a double newline
    let header_and_remainder = transmission.split("\r\n\r\n").collect::<Vec<_>>();
    let identifier = &header_and_remainder[0][5..];

    // Each message is separated by a carriage return and a line feed
    let mut messages = header_and_remainder[1].split("\r\n").collect::<Vec<_>>();

    // The last two entries will always be the end of message indicator and a carriage return with newline, which can safely be ignored
    messages.truncate(messages.len() - 2);

    for message in &messages {
        let obis_reference = obis::ObisReference::from_message(&message);

        if obis_reference.is_some() {
            let value = obis::ObisReference::get_value(&message);
            let ref_name = obis_reference.unwrap().to_string();

            if !value.trim().is_empty() {
                println!("{: <48}\t{}", ref_name, value);
            }
        }
    }

    None
}
