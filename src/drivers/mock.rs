// Diver mock for testing
use error_stack::Result;
use std::fmt;
use std::{error::Error, fmt::Display};

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
pub fn get_issues(_board_id: &str) -> Result<Vec<String>, MockDriverError> {
    Ok(vec![
        "GBC-1".to_string(),
        "GBC-2".to_string(),
        "GBC-3".to_string(),
        "GBC-4".to_string(),
        "GBC-5".to_string(),
        "GBC-6".to_string(),
        "GBC-7".to_string(),
        "GBC-8".to_string(),
        "GBC-9".to_string(),
    ])
}
