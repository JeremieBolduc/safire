use async_trait::async_trait;
use clap::Parser;
use rpassword;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use super::subcommand::SubcommandHandler;
use crate::utils::constants::DECRYPTED_FILE_EXT;
use crate::utils::gpg::{get_gpg_recipient, GpgManager};
use crate::utils::paths::app_root;

#[derive(Parser, Debug)]
pub struct AddArgs {
    pub path: String,
}

pub struct AddHandler {
    path: String,
}

impl AddHandler {
    pub fn new(args: AddArgs) -> Self {
        AddHandler {
            path: args.path.to_owned(),
        }
    }
}

#[async_trait]
impl SubcommandHandler for AddHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let password = rpassword::read_password_from_tty(Some(&format!(
            "Enter the password for {}: ",
            &self.path
        )))?;

        let store_path = app_root().join(&self.path);
        let decrypted_file_name = format!("{}.{}", self.path.replace("/", "-"), DECRYPTED_FILE_EXT);
        let decrypted_file_path = store_path.join(&decrypted_file_name);

        if let Err(err) = fs::create_dir_all(&store_path) {
            return Err(err.into());
        }

        let gpg_recipient = get_gpg_recipient()?;
        let mut gpg_manager = GpgManager::new(&gpg_recipient);

        let mut file = File::create(&decrypted_file_path)?;

        file.write_all(password.as_bytes())?;

        gpg_manager.encrypt_file(&decrypted_file_path)?;
        println!("Created a store for {} using the given password", self.path);

        Ok(None)
    }
}
