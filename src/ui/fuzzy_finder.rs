use fuzzy_finder::item::Item;
use std::io::Write;

// fuzzy finder errors
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

// Render a fuzzy_finder interface and returs a result or an error.
pub fn render(issues: Vec<String>) -> Result<String, Error> {
    // Create a list of items from the list of issues
    let items: Vec<Item<String>> = issues
        .iter()
        .map(|issue| Item::new(issue.to_string(), issue.to_string()))
        .collect();

    // Prompt the user to select an issue from the list of issues
    match fuzzy_finder::FuzzyFinder::find(items, 8) {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Err(Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No issue selected",
        ))),
        Err(e) => {
            let stderr = &mut std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "error: {}", e).expect(errmsg);
            Err(Error::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error writing to stderr",
            )))
        }
    }
}
