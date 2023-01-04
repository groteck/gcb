// CLI tool that creates a git branch from a fuzzy finder interface with a list
// of issues from a Jira board.

mod drivers;
mod git;
mod global_config;
mod ui;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use global_config::Config;
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

fn main() {
    let opts: Opts = Opts::parse();
    let mut config = Config::load().unwrap();

    match opts.command {
        Commands::GlobalConfig { command } => match command {
            GlobalConfigCommands::AddCredentials => {
                let credentials = ui::get_credentials();
                config.create_or_update_credentials(credentials).unwrap();
            }
            GlobalConfigCommands::Display => {
                config.print();
            }
        },
        Commands::Init {} => {
            println!("Init was called");
        }
        Commands::New { path } => {
            let credentials_url = &config
                .get_credentials("https://jira.atlassian.com".to_string())
                .unwrap()
                .url;
            let issues = drivers::mock::get_issues(credentials_url).unwrap_or_default();

            // TODO: Handle the branch name formatting
            // let branch_name = format!(
            //     "{}-{}",
            //     issue_key,
            //     jira::jira_get_issue_summary(&issue).replace(" ", "-")
            // );

            match fuzzy_finder::render(issues) {
                Ok(issue) => {
                    git::branch_create(path, issue.clone());
                    println!("Created branch {}", issue);
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
