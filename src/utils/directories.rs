use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};

use super::paths::get_app_path;

pub fn find_directories_in(root: &Path, query: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result = Vec::new();

    find_directories_in_recursive(root, query, &mut result)?;

    Ok(result)
}

fn find_directories_in_recursive(
    current_path: &Path,
    search_string: &str,
    result: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path.display().to_string().contains(search_string) {
                let app_path = get_app_path();
                let relative_path = path.strip_prefix(app_path)?.to_path_buf();

                result.push(relative_path);
            }

            find_directories_in_recursive(&path, search_string, result)?;
        }
    }

    Ok(())
}

pub fn find_files_in(
    root: &Path,
    predicate: &dyn Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result = Vec::new();

    get_files_in_recursive(root, predicate, &mut result)?;

    Ok(result)
}

fn get_files_in_recursive(
    current_path: &Path,
    predicate: &dyn Fn(&Path) -> bool,
    mut result: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            get_files_in_recursive(&path, predicate, &mut result)?;
        } else if path.is_file() {
            if predicate(&path) {
                result.push(path);
            }
        }
    }

    Ok(())
}
