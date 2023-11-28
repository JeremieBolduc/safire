use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::constants::APP_NAME;

pub fn find_directories(query: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Unable to determine home directory")?;
    let app_path = home_dir.join(APP_NAME);
    let mut result_directories = Vec::new();

    find_directories_recursive(&app_path, query, &mut result_directories)?;

    Ok(result_directories)
}

fn find_directories_recursive(
    current_path: &Path,
    search_string: &str,
    result_directories: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path.display().to_string().contains(search_string) {
                let home_dir = dirs::home_dir().ok_or("Unable to determine home directory")?;
                let app_path = home_dir.join(APP_NAME);
                let relative_path = path.strip_prefix(app_path)?.to_path_buf();
                result_directories.push(relative_path);
            }

            find_directories_recursive(&path, search_string, result_directories)?;
        }
    }

    Ok(())
}
