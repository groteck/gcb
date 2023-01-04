// Local project operations and local file configuration handlers

use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{error::Error, fmt::Display};

// Error ProjectConfig
#[derive(Debug)]
pub enum ProjectConfigError {
    InvalidPath,
    InvalidConfig,
}

impl Display for ProjectConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectConfigError::InvalidPath => write!(f, "Invalid path"),
            ProjectConfigError::InvalidConfig => write!(f, "Invalid config"),
        }
    }
}

impl Error for ProjectConfigError {}

// Struct to handle the project configuration
#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    pub identifier: String,
    pub url: String,
    pub branch_kinds: Vec<BranchKind>,
}

// Struct to handle branch kinds
// Default options are:
// Feature,
// Bugfix,
// Release,
// Hotfix,
// Support,
// Other,
#[derive(Serialize, Deserialize)]
pub struct BranchKind {
    pub kind: String,
    pub formatter: String,
}

// Function with the configuration file path
fn get_local_config_file() -> Result<File, ProjectConfigError> {
    let mut path = std::env::current_dir()
        .into_report()
        .change_context(ProjectConfigError::InvalidPath)?;
    path.push(".gbc"); // JSON formatted local file

    let file = std::fs::File::open(path)
        .into_report()
        .change_context(ProjectConfigError::InvalidConfig)
        .attach_lazy(|| "No configuration file found. Please run 'gbc init' to create one.")?;

    Ok(file)
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            identifier: String::from(""),
            url: String::from(""),
            branch_kinds: vec![
                BranchKind {
                    kind: String::from("feature"),
                    formatter: String::from("feature/{id}-{name}"),
                },
                BranchKind {
                    kind: String::from("bugfix"),
                    formatter: String::from("bugfix/{id}-{name}"),
                },
                BranchKind {
                    kind: String::from("release"),
                    formatter: String::from("release/{id}-{name}"),
                },
                BranchKind {
                    kind: String::from("hotfix"),
                    formatter: String::from("hotfix/{id}-{name}"),
                },
                BranchKind {
                    kind: String::from("support"),
                    formatter: String::from("support/{id}-{name}"),
                },
                BranchKind {
                    kind: String::from("other"),
                    formatter: String::from("other/{id}-{name}"),
                },
            ],
        }
    }
}

impl ProjectConfig {
    // Function to create a new project configuration
    pub fn new(
        identifier: String,
        url: String,
        branch_kinds: Option<Vec<BranchKind>>,
    ) -> ProjectConfig {
        ProjectConfig {
            identifier,
            url,
            branch_kinds: branch_kinds.unwrap_or(ProjectConfig::default().branch_kinds),
        }
    }

    // Function to load the project configuration
    pub fn load() -> Result<ProjectConfig, ProjectConfigError> {
        let file = get_local_config_file()?;

        serde_json::from_reader(file)
            .into_report()
            .change_context(ProjectConfigError::InvalidConfig)
            .attach_lazy(|| "Error while parsing configuration file.")
    }

    // Function to initialize the project configuration
    pub fn init(self) -> Result<ProjectConfig, ProjectConfigError> {
        match get_local_config_file() {
            Ok(_) => {
                println!(
                    "Configuration file already exists. Please remove it to create a new one."
                );

                Ok(self)
            }
            Err(_) => {
                let mut path = std::env::current_dir()
                    .into_report()
                    .change_context(ProjectConfigError::InvalidPath)?;
                path.push(".gbc"); // JSON formatted local file

                let file = std::fs::File::create(path)
                    .into_report()
                    .change_context(ProjectConfigError::InvalidConfig)?;

                serde_json::to_writer_pretty(file, &self)
                    .into_report()
                    .change_context(ProjectConfigError::InvalidConfig)
                    .attach_lazy(|| "Error while writing configuration file.")?;

                println!("Configuration with name .gbc created successfully.");

                Ok(self)
            }
        }
    }
}
