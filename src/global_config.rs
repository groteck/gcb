// Generate a global config file from the command line propt and save
// it to the user's configuration directory

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Access to the different APIs Bearer or Basic Auth
#[derive(Serialize, Deserialize)]
pub enum Access {
    Basic { username: String, password: String },
    Token { token: String },
}

// The different APIs that gbc can interact with
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub url: String,
    pub access: Access,
}

// Config file structure
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub credentials: Vec<Credentials>,
}

// Constant with the configuration file path
fn get_config_path() -> PathBuf {
    dirs::config_dir().unwrap().join("gbc/config.json")
}

impl Config {
    pub fn new() -> Config {
        Config {
            version: env!("CARGO_PKG_VERSION").to_string(),
            credentials: Vec::new(),
        }
    }

    fn read() -> Result<Config, Box<dyn std::error::Error>> {
        let config_file = std::fs::read_to_string(get_config_path())?;
        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config)
    }

    fn create() -> Result<Config, Box<dyn std::error::Error>> {
        let config = Config::new();
        let config_file = serde_json::to_string_pretty(&config)?;
        std::fs::write(get_config_path(), config_file)?;
        Ok(config)
    }

    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        if !get_config_path().exists() {
            return Config::create();
        }
        Config::read()
    }

    pub fn get_credentials(&self, url: String) -> Option<&Credentials> {
        self.credentials.iter().find(|c| c.url == url)
    }

    pub fn create_or_update_credentials(
        &mut self,
        credentials: Credentials,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let index = self
            .credentials
            .iter()
            .position(|c| c.url == credentials.url);
        match index {
            Some(i) => self.credentials[i] = credentials,
            None => self.credentials.push(credentials),
        }

        let config_file = serde_json::to_string_pretty(self)?;
        std::fs::write(get_config_path(), config_file)?;
        Ok(())
    }

    pub fn print(&self) {
        println!("{}", serde_json::to_string_pretty(&self).unwrap());
    }
}
