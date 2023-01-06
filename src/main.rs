// CLI tool that creates a git branch from a fuzzy finder interface with a list
// of issues from a Jira board.

mod drivers;
mod git;
mod global_config;
mod ui;

use clap::{Parser, Subcommand};
use global_config::Config;
use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};
use ui::fuzzy_finder;

// Command interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum GlobalConfigCommands {
    /// Create or update the global configuration file with the credentials
    /// provided
    AddCredentials,
    /// Display the global configuration file pretty printed
    Display,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Prompt an interface to set global configuration file
    GlobalConfig {
        #[command(subcommand)]
        command: GlobalConfigCommands,
    },
    /// Initialize a gbc project
    Init {},
    /// Display a fuzzy_finder interface to select an issue from
    /// the board and create a git branch from it
    New {
        #[arg(required = false)]
        path: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let opts: Opts = Opts::parse();

    Config::load()
        .and_then(|mut config| match opts.command {
            Commands::GlobalConfig { command } => match command {
                GlobalConfigCommands::AddCredentials => {
                    let credentials = ui::get_credentials();

                    config
                        .create_or_update_credentials(credentials)
                        .map(|_| ExitCode::SUCCESS)
                }
                GlobalConfigCommands::Display => config.print().map(|_| ExitCode::SUCCESS),
            },
            Commands::Init {} => {
                println!("Init was called");
                Ok(ExitCode::SUCCESS)
            }
            Commands::New { path } => {
                config
                    .get_credentials("https://jira.atlassian.com".to_string())
                    .map(|credentials| {
                        let issues =
                            drivers::mock::get_issues(&credentials.url).unwrap_or_default();

                        // TODO: Handle the branch name formatting
                        // let branch_name = format!(
                        //     "{}-{}",
                        //     issue_key,
                        //     jira::jira_get_issue_summary(&issue).replace(" ", "-")
                        // );

                        match fuzzy_finder::render(issues) {
                            Ok(issue) => {
                                git::branch_create(path, issue).unwrap_or_else(|e| {
                                    e.report();
                                });
                                ExitCode::SUCCESS
                            }
                            Err(e) => e.report(),
                        }
                    })
            }
        })
        .unwrap_or_else(|e| e.report())
}
