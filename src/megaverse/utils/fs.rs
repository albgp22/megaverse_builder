use std::error;
use std::fs;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// Tbh this function turned out to be a bit hacky. Sure there's a better way.
pub fn get_abs_path(file: &str) -> Result<String> {
    Ok(fs::canonicalize(PathBuf::from(file))?
        .as_path()
        .display()
        .to_string())
}
