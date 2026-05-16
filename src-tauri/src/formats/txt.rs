use super::FormatHandler;
use std::fs;
use std::path::Path;

pub struct TxtHandler;

impl FormatHandler for TxtHandler {
    fn extension(&self) -> &str {
        "txt"
    }

    fn count_matches(&self, path: &Path) -> Result<usize, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Ok(content.matches("67").count())
    }

    fn replace(&self, path: &Path) -> Result<usize, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let count = content.matches("67").count();
        if count > 0 {
            fs::write(path, content.replace("67", "69")).map_err(|e| e.to_string())?;
        }
        Ok(count)
    }
}
