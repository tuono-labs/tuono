use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub origin: Option<String>,
    pub port: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn get() -> io::Result<Config> {
        let config_file = read_to_string(PathBuf::from_iter([".tuono", "config", "config.json"]))?;

        serde_json::from_str(&config_file)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
