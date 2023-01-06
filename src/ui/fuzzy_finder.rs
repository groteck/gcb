use error_stack::{Report, Result};
use fuzzy_finder::item::Item;
use std::error::Error;
use std::fmt::{self, Display};
use std::io::Write;

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
pub fn render(issues: Vec<String>) -> Result<String, FuzzyFinderError> {
    // Create a list of items from the list of issues
    let items: Vec<Item<String>> = issues
        .iter()
        .map(|issue| Item::new(issue.to_string(), issue.to_string()))
        .collect();

    // Prompt the user to select an issue from the list of issues
    match fuzzy_finder::FuzzyFinder::find(items, 8) {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Err(Report::new(FuzzyFinderError::Empty)),
        Err(e) => {
            let stderr = &mut std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "error: {}", e).expect(errmsg);
            Err(Report::new(FuzzyFinderError::Other(e)))
        }
    }
}
