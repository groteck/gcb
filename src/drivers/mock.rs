// Diver mock for testing

// Driver Errors
#[derive(Debug)]
pub struct Error(String);

// list of issues from a board.
pub fn get_issues(_board_id: &str) -> Result<Vec<String>, Error> {
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
