use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use std::fs::File;
use std::io::{self};

use super::command_handler::CommandHandler;
use crate::data::store::Store;
use crate::utils::paths::get_app_path;

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

impl CommandHandler for CopyHandler {
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>> {
        let file_name = format!("{}.json", self.path.replace("/", "-"));
        let store_path = get_app_path().join(&self.path);
        let file_path = get_app_path().join(&self.path).join(file_name);

        if !store_path.exists() {
            return Ok(Some(format!("Could not find store for {}", self.path)));
        }

        if !file_path.exists() {
            return Ok(Some(format!(
                "Could not find the data file for {}",
                self.path
            )));
        }

        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);

        let store: Store = match serde_json::from_reader(reader) {
            Ok(store) => store,
            Err(_) => {
                return Err(format!(
                    "Error deserializing JSON from file: {}",
                    file_path.display()
                )
                .into());
            }
        };

        copy_to_clipboard(store.password)?;

        println!("{} store password copied to clipboard", self.path);

        Ok(None)
    }
}

fn copy_to_clipboard(data: String) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(data)?;
    Ok(())
}
