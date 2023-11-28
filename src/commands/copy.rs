use crate::commands::command_handler::CommandHandler;
use crate::entities::store::Store;
use crate::utils::constants::APP_NAME;
use crate::utils::find::find_directories;
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use std::fs::File;
use std::io::{self};

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
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.json", self.path.replace("/", "-"));
        let home_dir = dirs::home_dir().ok_or("Unable to determine home directory")?;
        let file_path = home_dir.join(APP_NAME).join(&self.path).join(file_name);

        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("File not found for store {}", self.path);

                let find_result = find_directories(&self.path);
                if let Ok(directories) = find_result {
                    println!("Here are some suggestions based on the input:");
                    directories.iter().for_each(|x| {
                        println!("{}", x.display());
                    });
                }

                return Ok(());
            }
        };

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

        Ok(())
    }
}

fn copy_to_clipboard(data: String) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(data)?;
    Ok(())
}
