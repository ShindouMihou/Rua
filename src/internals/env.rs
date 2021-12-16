use std::path::Path;
use std::fs::read_to_string;
use std::env::VarError;

/// Reads the .env file located in the root directory.
pub(crate) fn read() {
    if Path::new(".env").exists() {
        let contents = read_to_string(".env").unwrap();

        contents.split("\n").for_each(|s| {
            if s.contains("=") {
                let r: Vec<&str> = s.split("=").collect();
                std::env::set_var(r[0], r[1]);
            }
        });
    }
}

/// Proxies from std::env::var to make it easier to access.
pub(crate) fn get(key: &str) -> Result<String, VarError> {
    std::env::var(key)
}