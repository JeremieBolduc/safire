use clap::Parser;
use dirs;
use rand::Rng;
use serde_json::to_writer_pretty;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use super::command_handler::CommandHandler;
use crate::entities::store::Store;
use crate::utils::constants::APP_NAME;

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    pub path: String,
    /// Length of the generated password
    #[clap(short, long, default_value_t = 25)]
    pub length: u32,
}

pub struct GenerateHandler {
    path: String,
    length: u32,
}

impl GenerateHandler {
    pub fn new(args: GenerateArgs) -> Self {
        return GenerateHandler {
            path: args.path.to_owned(),
            length: args.length,
        };
    }
}

impl CommandHandler for GenerateHandler {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()[]{}/`;:,.<>-_+=";

        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let index = rng.gen_range(0..CHARSET.len());
                CHARSET[index] as char
            })
            .collect();

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
        println!(
            "Created a store for {} using the generated password",
            self.path
        );

        Ok(())
    }
}

fn create_directories(path: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)?;

    Ok(())
}
