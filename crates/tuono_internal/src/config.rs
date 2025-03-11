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

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "localhost".to_string(),
            origin: None,
            port: 3000,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.server.host, "localhost".to_string());
        assert_eq!(config.server.origin, None);
        assert_eq!(config.server.port, 3000);
    }
}
