use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use super::paths::get_app_path;

pub fn find_directories(query: &str, from: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result_directories = Vec::new();

    find_directories_recursive(from, query, &mut result_directories)?;

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
                let app_path = get_app_path();
                let relative_path = path.strip_prefix(app_path)?.to_path_buf();

                result_directories.push(relative_path);
            }

            find_directories_recursive(&path, search_string, result_directories)?;
        }
    }

    Ok(())
}

pub fn create_directories(path: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)?;

    Ok(())
}
