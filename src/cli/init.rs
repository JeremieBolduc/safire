use async_trait::async_trait;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::utils::constants::GPG_RECIPIENT_FILENAME;
use crate::utils::paths::app_root;

use super::subcommand::SubcommandHandler;

#[derive(Parser, Debug)]
pub struct InitArgs {
    gpg_recipient: String,
}

pub struct InitHandler {
    gpg_recipient: String,
}

impl InitHandler {
    pub fn new(args: InitArgs) -> Self {
        InitHandler {
            gpg_recipient: args.gpg_recipient.to_owned(),
        }
    }
}

#[async_trait]
impl SubcommandHandler for InitHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let app_root = app_root();
        let file_path = app_root.join(GPG_RECIPIENT_FILENAME);
        let mut file = File::create(file_path)?;

        file.write_all(self.gpg_recipient.as_bytes())?;

        Ok(None)
    }
}
