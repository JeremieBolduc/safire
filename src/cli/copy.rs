use async_trait::async_trait;
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::time::sleep;

use super::subcommand::SubcommandHandler;
use crate::utils::constants::ENCRYPTED_FILE_EXT;
use crate::utils::gpg::{get_gpg_recipient, GpgManager};
use crate::utils::paths::app_root;

#[derive(Parser, Debug)]
pub struct CopyArgs {
    path: String,
}

pub struct CopyHandler {
    path: String,
}

impl CopyHandler {
    pub fn new(args: CopyArgs) -> Self {
        CopyHandler {
            path: args.path.to_owned(),
        }
    }
}

#[async_trait]
impl SubcommandHandler for CopyHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let encrypted_file_name = format!("{}.{}", self.path.replace("/", "-"), ENCRYPTED_FILE_EXT);
        let store_path = app_root().join(&self.path);
        let encrypted_file_path = store_path.join(&encrypted_file_name);

        if !store_path.exists() {
            return Ok(Some(format!("Could not find store for {}", self.path)));
        }

        let gpg_recipient = get_gpg_recipient()?;
        let mut gpg_manager = GpgManager::new(&gpg_recipient);

        let decrypted_file_path = gpg_manager.decrypt_file(&encrypted_file_path)?;

        if !decrypted_file_path.exists() {
            return Ok(Some(format!(
                "Could not find the data file for {}",
                self.path
            )));
        }

        let file = File::open(&decrypted_file_path)?;

        let password = read_password_from_file(&file)?;

        copy_to_clipboard(password)?;

        println!("{} store password copied to clipboard", self.path);

        tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;
            clear_clipboard().unwrap_or_else(|err| eprintln!("Error clearing clipboard: {}", err));
        })
        .await?;

        Ok(None)
    }
}

fn read_password_from_file(file: &std::fs::File) -> Result<String, Box<dyn std::error::Error>> {
    let reader = io::BufReader::new(file);

    match reader.lines().next() {
        Some(Ok(line)) if !line.trim().is_empty() => Ok(line.trim().to_string()),
        Some(_) => Err("Password is empty or contains only whitespace".into()),
        None => Err("Error reading password from file".into()),
    }
}

fn copy_to_clipboard(data: String) -> Result<(), Box<dyn Error>> {
    let mut context: ClipboardContext = ClipboardProvider::new()?;
    context.set_contents(data)?;

    Ok(())
}

fn clear_clipboard() -> Result<(), Box<dyn Error>> {
    let mut context: ClipboardContext = ClipboardProvider::new()?;
    context.set_contents("".to_owned())?;

    Ok(())
}
