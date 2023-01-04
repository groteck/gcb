// CLI tool that creates a git branch from a fuzzy finder interface with a list
// of issues from a Jira board.

mod drivers;
mod features;
mod git;
mod ui;

use clap::{Parser, Subcommand};
use error_stack::Result;
use features::global_config::Config;
use features::init::ProjectConfig;
use std::{error::Error, fmt::Display};
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
        /// You can select the type of branch you want to create, a default list
        /// will be created by default in your project configuration file.
        type_of_branch: Option<String>,
        #[arg(required = false)]
        path: Option<PathBuf>,
    },
}

// CLI error
#[derive(Debug)]
pub struct CLIError {}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Global error")
    }
}

impl Error for CLIError {}

// Helper function to update the error context
fn update_err_ctx<T, E>(result: Result<T, E>) -> Result<T, CLIError> {
    result.map_err(|e| {
        let error_attachment = e.to_string();
        e.change_context(CLIError {})
            .attach_printable(error_attachment)
    })
}

// Route the command to the right function
fn router(opts: Opts) -> Result<(), CLIError> {
    let mut config = update_err_ctx(Config::load())?;

    match opts.command {
        Commands::GlobalConfig { command } => update_err_ctx(match command {
            GlobalConfigCommands::AddCredentials => {
                let credentials = ui::get_credentials();

                config.create_or_update_credentials(credentials)
            }
            GlobalConfigCommands::Display => config.print(),
        }),
        Commands::Init {} => {
            let project_config = ui::get_project_config();
            update_err_ctx(project_config.init()).map(|_| ())
        }
        Commands::New {
            path,
            type_of_branch,
        } => {
            let project_config = update_err_ctx(ProjectConfig::load())?;
            let credentials = update_err_ctx(config.get_credentials(project_config.url))?;
            let issues = drivers::mock::get_issues(&credentials.url).unwrap_or_default();
            let branch_formatter = project_config
                .branch_kinds
                .iter()
                .find(|branch_kind| branch_kind.kind == type_of_branch.clone().unwrap_or_default())
                .map(|branch_kind| branch_kind.formatter.clone())
                .unwrap_or_else(|| String::from("feature/{id}-{name}"));
            let issue = update_err_ctx(fuzzy_finder::render(issues))?;

            let branch_name = branch_formatter
                .replace("{id}", &issue.id)
                .replace("{name}", &issue.title.replace(' ', "-"));

            update_err_ctx(git::branch_create(path, branch_name).map(|_| Ok(())))?
        }
    }
}

fn main() -> ExitCode {
    let opts: Opts = Opts::parse();

    router(opts).map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e:?}");

        e.report()
    })
}
