use dirs::home_dir;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use crossterm::style::Stylize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub spells_dir: String,
}

impl Config {

    pub fn from(config_path: &str) -> Result<Config, io::Error> {
        println!(
            "{} {}",
            "Using config:".green(),
            config_path.bold().underlined()
        );

        let config_str = match fs::read_to_string(config_path) {
            Ok(v) => v,
            Err(e) => {
                return Err(io::Error::new(
                    e.kind(),
                    format!("{}: {}", config_path, e.to_string()),
                ));
            }
        };

        let config: Config = match serde_json::from_str(config_str.as_str()) {
            Ok(v) => v,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}: {}", config_path, e.to_string()),
                ));
            }
        };

        Ok(config)
    }

    pub fn new() -> Result<Config, io::Error> {
        Config::from(Config::get_config_path()?.as_str())
    }

    pub fn get_config_path() -> Result<String, io::Error> {

        let config_paths = Config::get_default_paths();
        for path in config_paths {
            if Path::new(path.as_str()).exists() {
                return Ok(path);
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Config file not found."))
    }

    fn get_default_paths() -> Vec<String> {
        let mut default_paths: Vec<String> = Vec::new();

        default_paths.push(String::from("./config/config.json"));
        default_paths.push(format!("{}/{}", home_dir().unwrap().to_str().unwrap(), ".config/fspell/config.json"));

        default_paths
    }
}
