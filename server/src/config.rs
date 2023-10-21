use serde::{Deserialize, Serialize};
use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub name: String,
    pub debug: bool,
    pub locale: String,
    pub ip: String,
    pub port : u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "Latifa".to_string(),
            debug: false,
            locale: "en".to_string(),
            ip: "0.0.0.0".to_string(),
            port: 31921,
        }
    }
}

impl Config {
    pub fn load(path: &std::path::Path) -> Self {
        if !path.exists() {
            use std::io::prelude::*;
            log::warn("no config exists yet! creating a new one with defaults now!");
            let mut config_file = std::fs::File::create("./config.json").unwrap();
            let config = Config::default();
            let config_str = serde_json::to_vec_pretty(&config).unwrap();
            config_file.write_all(&config_str).unwrap();
        }
        let config_file = std::fs::File::open(path).unwrap();
        let read = std::io::BufReader::new(config_file);
        log::info("config loaded and read!");
        serde_json::from_reader(read).unwrap()
    }
}