// Generate a global config file from the command line propt and save
// it to the user's configuration directory

use error_stack::{Report, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::{error::Error, fmt::Display};

// Access to the different APIs Bearer or Basic Auth
#[derive(Serialize, Deserialize, Debug)]
pub enum Access {
    Basic { username: String, password: String },
    Token { token: String },
}

// The different APIs that gbc can interact with
#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub url: String,
    pub access: Access,
}

// Config file structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub credentials: Vec<Credentials>,
}

// Constant with the configuration file path
fn get_config_path() -> Result<PathBuf, ConfigError> {
    dirs::config_dir()
        .ok_or_else(|| Report::new(ConfigError::NoConfigDirectoryAvailable))
        .map(|path| path.join("gbc/config.json"))
}

// Errors that can occur when reading the configuration file
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    FileNotCreated,
    CredentialsNotFound,
    ParseError,
    SerializationError,
    NoConfigDirectoryAvailable,
}

impl Display for ConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound => fmt.write_str("Configuration file not found"),
            ConfigError::FileNotCreated => fmt.write_str("Configuration file not created"),
            ConfigError::CredentialsNotFound => fmt.write_str("Credentials not found"),
            ConfigError::ParseError => fmt.write_str("Error parsing configuration file"),
            ConfigError::SerializationError => fmt.write_str("Error serializing configuration"),
            ConfigError::NoConfigDirectoryAvailable => {
                fmt.write_str("Error we did not find a configuration directory")
            }
        }
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn new() -> Config {
        Config {
            version: env!("CARGO_PKG_VERSION").to_string(),
            credentials: Vec::new(),
        }
    }

    fn read() -> Result<Config, ConfigError> {
        std::fs::read_to_string(get_config_path()?)
            .map_err(|_| Report::new(ConfigError::FileNotFound))
            .and_then(|s| {
                serde_json::from_str(&s).map_err(|_| Report::new(ConfigError::ParseError))
            })
    }

    fn create() -> Result<Config, ConfigError> {
        serde_json::to_string_pretty(&Config::new())
            .map_err(|_| Report::new(ConfigError::SerializationError))
            .and_then(|s| {
                std::fs::write(get_config_path()?, s).map_err(|_| {
                    Report::new(ConfigError::FileNotCreated).attach_printable(format!(
                        "Could not create configuration file at {}",
                        get_config_path().unwrap_or_default().display()
                    ))
                })
            })
            .and_then(|_| Config::read())
    }

    pub fn load() -> Result<Config, ConfigError> {
        if !get_config_path()?.exists() {
            return Config::create();
        }
        Config::read()
    }

    pub fn get_credentials(&self, url: String) -> Result<&Credentials, ConfigError> {
        self.credentials
            .iter()
            .find(|c| c.url == url)
            .ok_or_else(|| {
                Report::new(ConfigError::CredentialsNotFound)
                    .attach_printable(format!("Credentials for {} not found", url))
            })
    }

    pub fn create_or_update_credentials(
        &mut self,
        credentials: Credentials,
    ) -> Result<(), ConfigError> {
        let index = self
            .credentials
            .iter()
            .position(|c| c.url == credentials.url);
        match index {
            Some(i) => self.credentials[i] = credentials,
            None => self.credentials.push(credentials),
        }

        serde_json::to_string_pretty(self)
            .map_err(|_| {
                Report::new(ConfigError::ParseError)
                    .attach_printable(format!("Could not serialize configuration file {:?}", self))
            })
            .and_then(|s| {
                std::fs::write(get_config_path()?, s).map_err(|_| {
                    Report::new(ConfigError::FileNotCreated).attach_printable(format!(
                        "Could not write configuration file at {}",
                        get_config_path().unwrap_or_default().display()
                    ))
                })
            })
            .map(|_| ())
    }

    pub fn print(&self) -> Result<(), ConfigError> {
        // TODO: hide the passwords in the output
        serde_json::to_string_pretty(&self)
            .map(|s| {
                println!("{}", s);
            })
            .map_err(|_| {
                Report::new(ConfigError::ParseError)
                    .attach_printable(format!("Could not serialize configuration {:?}", self))
            })
    }
}
