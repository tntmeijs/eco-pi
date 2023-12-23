use std::{fs, time::Duration};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

const SETTINGS_FILE_PATH: &str = "./settings.yaml";

#[derive(Deserialize, Serialize)]
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
    let mut buffer: Vec<u8> = vec![0; 2048];

    loop {
        match port.read(buffer.as_mut_slice()) {
            Ok(t) => info!("{}", String::from_utf8_lossy(&buffer[..t])),
            Err(e) => error!("Failed to read: {}", e)
        }
    }
}
