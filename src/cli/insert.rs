use async_trait::async_trait;
use clap::Parser;
use rpassword;
use serde_json::to_writer_pretty;
use std::error::Error;
use std::fs::{self, File};

use super::command_handler::CommandHandler;
use crate::data::store::Store;
use crate::utils::constants::DECRYPTED_FILE_EXT;
use crate::utils::paths::get_app_path;

#[derive(Parser, Debug)]
pub struct InsertArgs {
    pub path: String,
}

pub struct InsertHandler {
    path: String,
}

impl InsertHandler {
    pub fn new(args: InsertArgs) -> Self {
        InsertHandler {
            path: args.path.to_owned(),
        }
    }
}

#[async_trait]
impl CommandHandler for InsertHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let password = rpassword::read_password_from_tty(Some(&format!(
            "Enter the password for {}: ",
            &self.path
        )))?;

        let store_path = get_app_path().join(&self.path);
        let file_name = format!("{}.{}", self.path.replace("/", "-"), DECRYPTED_FILE_EXT);
        let file_path = store_path.join(&file_name);

        if let Err(err) = fs::create_dir_all(&store_path) {
            eprintln!("Error creating directories: {}", err);
            return Err(err.into());
        }

        let mut file = File::create(file_path)?;
        let store = Store::new(&password, None);

        to_writer_pretty(&mut file, &store)?;
        println!("Created a store for {} using the given password", self.path);

        Ok(None)
    }
}
