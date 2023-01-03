// CLI tool that creates a git branch from a fuzzy finder interface with a list
// of issues from a Jira board.

// use std::convert::TryInto;
// use std::fs;
// use std::io::Write;

use clap::Parser;
// use fuzzy_finder::item::Item;

mod git;

// Command interface
#[derive(Parser)]
#[clap(version = "0.1", author = "Juan Fraire")]
struct Opts {
    #[clap(short, long)]
    branch: String,
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

    let opts: Opts = Opts::parse();
    let branch_name = opts.branch.to_string();

    git::branch_create(branch_name.clone());

    println!("Created branch {}", branch_name);
}
