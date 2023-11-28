use clap::Parser;
use rand::Rng;
use serde_json::to_writer_pretty;
use std::error::Error;
use std::fs::File;

use super::command_handler::CommandHandler;
use crate::data::store::Store;
use crate::utils::directories::create_directories;
use crate::utils::paths::get_app_path;

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
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>> {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()[]{}/`;:,.<>-_+=";

        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let index = rng.gen_range(0..CHARSET.len());
                CHARSET[index] as char
            })
            .collect();

        let store_path = get_app_path().join(&self.path);
        let file_name = format!("{}.json", self.path.replace("/", "-"));
        let file_path = store_path.join(&file_name);

        if let Err(err) = create_directories(&store_path) {
            eprintln!("Error creating directories: {}", err);
            return Err(err.into());
        }

        let mut file = File::create(file_path)?;
        let store = Store::new(&password, None);

        to_writer_pretty(&mut file, &store)?;
        println!(
            "Created a store for {} using the generated password",
            self.path
        );

        Ok(None)
    }
}
