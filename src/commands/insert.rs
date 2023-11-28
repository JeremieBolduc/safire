use clap::Parser;
use dirs;
use rpassword;
use serde_json::to_writer_pretty;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use super::command_handler::CommandHandler;
use crate::entities::store::Store;
use crate::utils::constants::APP_NAME;

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

impl CommandHandler for InsertHandler {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let password = rpassword::read_password_from_tty(Some(&format!(
            "Enter the password for {}: ",
            &self.path
        )))?;

        let home_dir = dirs::home_dir().ok_or("Unable to determine home directory")?;
        let complete_path = home_dir.join(APP_NAME).join(&self.path);
        let file_name = format!("{}.json", self.path.replace("/", "-"));

        if let Err(err) = create_directories(&complete_path) {
            eprintln!("Error creating directories: {}", err);
            return Err(err.into());
        }

        let mut file = File::create(&complete_path.join(&file_name))?;
        let store = Store::new(&password, None);

        to_writer_pretty(&mut file, &store)?;
        println!("Created a store for {} using the given password", self.path);

        Ok(())
    }
}

fn create_directories(path: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)?;

    Ok(())
}
