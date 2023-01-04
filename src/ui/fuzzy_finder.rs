use error_stack::{Report, Result};
use fuzzy_finder::item::Item;
use std::error::Error;
use std::fmt::{self, Display};
use std::io::Write;

use crate::drivers::issue::Issue;

// fuzzy finder errors
#[derive(Debug)]
pub enum FuzzyFinderError {
    Empty,
    Other(anyhow::Error),
}

impl Display for FuzzyFinderError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for FuzzyFinderError {}

// Render a fuzzy_finder interface and returs a result or an error.
pub fn render(issues: Vec<Issue>) -> Result<Issue, FuzzyFinderError> {
    // Create a list of items from the list of issues
    let items = issues
        .iter()
        .map(|issue| Item::new(issue.id.to_string(), issue.title.to_string()))
        .collect();

    // Prompt the user to select an issue from the list of issues
    match fuzzy_finder::FuzzyFinder::find(items, 8) {
        Ok(Some(result)) => {
            // Get the issue from the list of issues
            let issue = issues.into_iter().find(|issue| issue.id == result).unwrap();

            Ok(issue)
        }
        Ok(None) => Err(Report::new(FuzzyFinderError::Empty)),
        Err(e) => {
            let stderr = &mut std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "error: {}", e).expect(errmsg);
            Err(Report::new(FuzzyFinderError::Other(e)))
        }
    }
}
