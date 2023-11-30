use async_trait::async_trait;
use clap::Parser;
use std::error::Error;
use std::process::Command;

use super::command_handler::CommandHandler;
use crate::utils::constants::ENCRYPTED_FILE_EXT;
use crate::utils::gpg::{get_gpg_recipient, GpgManager};
use crate::utils::paths::get_app_path;

#[derive(Parser, Debug)]
pub struct EditArgs {
    path: String,
    #[clap(short, long, default_value = "vi")]
    pub editor: String,
}

pub struct EditHandler {
    path: String,
    editor: String,
}

impl EditHandler {
    pub fn new(args: EditArgs) -> Self {
        EditHandler {
            path: args.path.to_owned(),
            editor: args.editor.to_owned(),
        }
    }
}

#[async_trait]
impl CommandHandler for EditHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let encrypted_file_name = format!("{}.{}", self.path.replace("/", "-"), ENCRYPTED_FILE_EXT);
        let store_path = get_app_path().join(&self.path);
        let encrypted_file_path = store_path.join(&encrypted_file_name);

        let gpg_recipient = get_gpg_recipient()?;
        let mut gpg_manager = GpgManager::new(&gpg_recipient);

        let decrypted_file_path = gpg_manager.decrypt_file(&encrypted_file_path)?;

        if !decrypted_file_path.exists() {
            return Ok(Some(format!(
                "Could not find the data file for {}",
                self.path
            )));
        }

        let status = Command::new(&self.editor)
            .arg(&decrypted_file_path)
            .status()?;

        println!("Editing store {} with {}", &self.path, self.editor);

        if status.success() {
            Ok(None)
        } else {
            Err("Editor process exited with an error".into())
        }
    }
}
