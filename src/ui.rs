// CLI UI components
pub mod fuzzy_finder;

use crate::features::{
    global_config::{Access, Credentials},
    init::ProjectConfig,
};

fn prompt(question: &str) -> String {
    let mut answer = String::new();
    println!("{}", question);
    std::io::stdin().read_line(&mut answer).unwrap_or_default();
    answer.trim().to_string()
}

// Helper function that creates a new Credentials struct from the user input
// and updates the Configurations file with the new credentials
pub fn get_credentials() -> Credentials {
    let url = prompt("Enter the URL of the API");
    let access = prompt("Enter the access type (basic or token)");

    match access.as_str() {
        "basic" => {
            let username = prompt("Enter the username");
            let password = prompt("Enter the password");
            Credentials {
                url,
                access: Access::Basic { username, password },
            }
        }
        "token" => {
            let token = prompt("Enter the token");
            Credentials {
                url,
                access: Access::Token { token },
            }
        }
        _ => {
            println!("Invalid access type");
            std::process::exit(1);
        }
    }
}

// Helper function that initialize the project configuration file from the user input
pub fn get_project_config() -> ProjectConfig {
    let identifier = prompt("Enter the project/board identifier");
    let url = prompt("Enter the project API URL");

    ProjectConfig::new(identifier, url, None)
}
