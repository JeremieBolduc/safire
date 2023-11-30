use async_trait::async_trait;
use clap::Parser;
use rand::Rng;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use super::command_handler::CommandHandler;
use crate::utils::constants::DECRYPTED_FILE_EXT;
use crate::utils::gpg::{get_gpg_recipient, GpgManager};
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

#[async_trait]
impl CommandHandler for GenerateHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*-_+=";
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let index = rng.gen_range(0..CHARSET.len());
                CHARSET[index] as char
            })
            .collect();

        let store_path = get_app_path().join(&self.path);
        let decrypted_file_name = format!("{}.{}", self.path.replace("/", "-"), DECRYPTED_FILE_EXT);
        let decrypted_file_path = store_path.join(&decrypted_file_name);

        if let Err(err) = fs::create_dir_all(&store_path) {
            eprintln!("Error creating directories: {}", err);
            return Err(err.into());
        }

        let gpg_recipient = get_gpg_recipient()?;
        let mut gpg_manager = GpgManager::new(&gpg_recipient);

        let mut file = File::create(&decrypted_file_path)?;

        file.write_all(password.as_bytes())?;

        gpg_manager.encrypt_file(&decrypted_file_path)?;

        println!(
            "Created a store for {} using the generated password",
            self.path
        );

        Ok(None)
    }
}
