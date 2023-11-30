use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};

use super::constants::APP_NAME;

pub fn filtered_search(
    root: &Path,
    predicate: &dyn Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result = Vec::new();

    filtered_search_recursive(root, predicate, &mut result)?;

    Ok(result)
}

fn filtered_search_recursive(
    current_path: &Path,
    predicate: &dyn Fn(&Path) -> bool,
    mut result: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if predicate(&path) {
            result.push(path.clone());
        }
        if path.is_dir() {
            filtered_search_recursive(&path, predicate, &mut result)?;
        }
    }

    Ok(())
}

pub fn app_root() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(format!(".{}", APP_NAME))
}

pub fn to_short_path(path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let stripped_path = path
        .strip_prefix(app_root())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    if path.is_file() {
        Ok(stripped_path
            .parent()
            .ok_or("Invalid file path")?
            .to_path_buf())
    } else {
        Ok(stripped_path.to_path_buf())
    }
}
