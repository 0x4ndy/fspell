use std::{fs, io};
use dirs::home_dir;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub spells_dir: String,
}

impl Config {
    pub fn get_config_path() -> String {
        let config_path = format!("{}/{}", home_dir().unwrap().to_str().unwrap(), DEFAULT_CONFIG_PATH);

        config_path
    }

    pub fn from(config_path: &str) -> Result<Config, io::Error> {
        let config_str = match fs::read_to_string(config_path) {
            Ok(v) => v,
            Err(e) => {
                return Err(io::Error::new(
                        e.kind(),
                        format!("{}: {}", config_path, e.to_string())
                        ));
            },
        };

        //let config: Config = serde_json::from_str(config_str.as_str())?;
        let config: Config = match serde_json::from_str(config_str.as_str()) {
            Ok(v) => v,
            Err(e) => {
                return Err(io::Error::new(
                        io::ErrorKind::InvalidData, 
                        format!("{}: {}", config_path, e.to_string())
                        ));
            }
        };

        Ok(config)
    }

    pub fn new() -> Result<Config, io::Error> {
        Config::from(Config::get_config_path().as_str())
    }
}
 
const DEFAULT_CONFIG_PATH: &str = ".config/fspell/config.json";
