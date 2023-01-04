// Diver mock for testing
use error_stack::Result;
use std::fmt;
use std::{error::Error, fmt::Display};

use super::issue::Issue;

// Driver Errors
#[derive(Debug)]
pub struct MockDriverError(String);

impl Display for MockDriverError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)
    }
}

impl Error for MockDriverError {}

// list of issues from a board.
pub fn get_issues(_board_id: &str) -> Result<Vec<Issue>, MockDriverError> {
    Ok(vec![
        Issue {
            id: "1".to_string(),
            title: "Issue 1".to_string(),
        },
        Issue {
            id: "2".to_string(),
            title: "Issue 2".to_string(),
        },
        Issue {
            id: "3".to_string(),
            title: "Issue 3".to_string(),
        },
        Issue {
            id: "4".to_string(),
            title: "Issue 4".to_string(),
        },
        Issue {
            id: "5".to_string(),
            title: "Issue 5".to_string(),
        },
        Issue {
            id: "6".to_string(),
            title: "Issue 6".to_string(),
        },
        Issue {
            id: "7".to_string(),
            title: "Issue 7".to_string(),
        },
        Issue {
            id: "8".to_string(),
            title: "Issue 8".to_string(),
        },
    ])
}
