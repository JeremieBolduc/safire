use async_trait::async_trait;
use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use regex::Regex;
use std::{
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use super::subcommand::SubcommandHandler;
use crate::utils::{
    gpg::{get_gpg_recipient, GpgManager},
    paths::{app_root, filtered_search, to_short_path},
};

#[derive(Parser, Debug)]
pub struct GrepArgs {
    pub regex: String,
}

pub struct GrepHandler {
    regex: String,
}

impl GrepHandler {
    pub fn new(args: GrepArgs) -> Self {
        GrepHandler {
            regex: args.regex.to_owned(),
        }
    }
}

fn get_line_match_outputs(
    file_path: &Path,
    regex: &Regex,
) -> Result<Option<(PathBuf, Vec<String>)>, Box<dyn Error>> {
    let file_content = fs::read_to_string(&file_path).unwrap_or_default();
    let mut outputs = Vec::new();

    for line in file_content.lines() {
        let mut line_match = String::new();
        let mut last_match_end = 0;

        for captures in regex.captures_iter(line) {
            let matched_text = captures.get(0).unwrap();
            let before_match = &line[last_match_end..matched_text.start()];
            let matched_part = matched_text.as_str();

            line_match.push_str(&format!(
                "{}{}",
                before_match,
                matched_part.bold().magenta()
            ));

            last_match_end = matched_text.end();
        }

        if last_match_end != 0 {
            line_match.push_str(&line[last_match_end..]);
            outputs.push(line_match);
        }
    }

    if !outputs.is_empty() {
        Ok(Some((file_path.to_path_buf(), outputs)))
    } else {
        Ok(None)
    }
}

#[async_trait]
impl SubcommandHandler for GrepHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let regex = Regex::new(&self.regex)?;

        let encrypted_file_paths = filtered_search(&app_root(), &|file_path| {
            if let Some(file_name) = file_path.file_name() {
                let os_str: &OsStr = file_name;
                return os_str.to_string_lossy().ends_with(".gpg");
            }

            false
        })?;

        let gpg_manager = Arc::new(Mutex::new(GpgManager::new(&get_gpg_recipient()?)));
        let decrypted_file_paths: Vec<_> = encrypted_file_paths
            .into_par_iter()
            .filter_map(|encrypted_file_path| {
                let gpg_manager = Arc::clone(&gpg_manager);
                let result = {
                    let mut manager = gpg_manager.lock().unwrap();
                    manager.decrypt_file(&encrypted_file_path).ok()
                };
                result
            })
            .collect();

        let line_match_outputs: Vec<_> = decrypted_file_paths
            .into_par_iter()
            .filter_map(|file_path| get_line_match_outputs(&file_path, &regex).ok())
            .filter_map(|x| x)
            .collect();

        line_match_outputs.into_iter().for_each(|x| {
            let (path, line_match_outputs) = x;
            if let Ok(store_path) = to_short_path(&path) {
                println!("{}:", store_path.display().to_string().cyan());

                line_match_outputs.iter().for_each(|line_match_output| {
                    println!("{}", line_match_output);
                })
            }
        });

        Ok(None)
    }
}
