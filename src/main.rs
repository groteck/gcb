// CLI tool that creates a git branch from a fuzzy finder interface with a list
// of issues from a Jira board.

// use std::convert::TryInto;
// use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use fuzzy_finder::item::Item;

mod drivers;
mod git;

// Command interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Prompt an interface to set global configuration file
    GlobalConfig {
        /// Sets a custom global config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    },
    /// Initialize a gbc project
    Init {
        /// Sets a custom project file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    },
    /// Display a fuzzy_finder interface to select an issue from
    /// the board and create a git branch from it
    New {},
}

fn create_fuzzy_finder(issues: Vec<String>) -> String {
    // Create a list of items from the list of issues
    let items: Vec<Item<String>> = issues
        .iter()
        .map(|issue| Item::new(issue.to_string(), issue.to_string()))
        .collect();

    // Prompt the user to select an issue from the list of issues
    let items_count = items.len();

    match fuzzy_finder::FuzzyFinder::find(items, items_count.try_into().unwrap()) {
        Ok(Some(result)) => result,
        Ok(None) => {
            panic!("Invalid result");
        }
        Err(e) => {
            std::io::stdout().flush().unwrap();
            panic!("Failed to find result: {}", e);
        }
    }
}

fn main() {
    // let issues = jira::jira_get_issues(args.jira_board_id);
    // let issue_keys = jira::jira_get_issue_keys(&issues);
    //
    // let issue_key = fuzzy_finder::fuzzy_find(issue_keys, |item| item.as_str())
    //     .expect("No issue selected")
    //     .to_string();
    //
    // let issue = jira::jira_get_issue(&issue_key);
    //
    // let branch_name = format!(
    //     "{}-{}",
    //     issue_key,
    //     jira::jira_get_issue_summary(&issue).replace(" ", "-")
    // );
    let issues = drivers::mock::get_issues("GBC");

    let opts: Opts = Opts::parse();

    match opts.command {
        Commands::GlobalConfig { config } => {
            println!("GlobalConfig");
            println!("{:?}", config);
        }
        Commands::Init { config } => {
            println!("Init");
            println!("{:?}", config);
        }
        Commands::New {} => {
            let item = create_fuzzy_finder(issues);
            println!();
            git::branch_create(item.clone());

            println!("Created branch {}", item);
        }
    }
}
