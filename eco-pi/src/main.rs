use std::{
    fs,
    process::{exit, ExitCode, ExitStatus},
};

use log::{error, warn};
use serde::{Deserialize, Serialize};

const SETTINGS_FILE_PATH: &str = "./settings.yaml";

#[derive(Deserialize, Serialize)]
struct Configuration {
    port_name: String,
    baud_rate: i32,
}

impl Default for Configuration {
    /// Creates a configuration with sensible default values
    fn default() -> Self {
        Self {
            port_name: "/dev/ttyUSB0".to_owned(),
            baud_rate: 115_200,
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
    todo!("application logic goes here")
}
