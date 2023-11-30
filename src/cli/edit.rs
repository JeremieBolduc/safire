use async_trait::async_trait;
use clap::Parser;
use colored::Colorize;
use std::error::Error;
use std::process::Command;

use super::subcommand::SubcommandHandler;
use crate::utils::constants::ENCRYPTED_FILE_EXT;
use crate::utils::gpg::{get_gpg_recipient, GpgManager};
use crate::utils::paths::app_root;

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
impl SubcommandHandler for EditHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let encrypted_file_name = format!("{}.{}", self.path.replace("/", "-"), ENCRYPTED_FILE_EXT);
        let store_path = app_root().join(&self.path);
        let encrypted_file_path = store_path.join(&encrypted_file_name);

        if !encrypted_file_path.exists() {
            return Ok(Some(format!(
                "Could not find a store for {}",
                self.path.to_string().cyan()
            )));
        }

        let gpg_recipient = get_gpg_recipient()?;
        let mut gpg_manager = GpgManager::new(&gpg_recipient);

        let decrypted_file_path = gpg_manager.decrypt_file(&encrypted_file_path)?;

        println!(
            "Editing store {} with {}",
            &self.path.cyan(),
            self.editor.green()
        );

        let mut editor_command = Command::new(&self.editor);
        editor_command.arg(&decrypted_file_path);

        if &self.editor == "code" {
            editor_command.arg("--wait");
        }

        let status = editor_command.status()?;

        if status.success() {
            Ok(None)
        } else {
            Err("Editor process exited with an error".into())
        }
    }
}
