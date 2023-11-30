use async_trait::async_trait;
use clap::Parser;
use std::error::Error;
use std::fs;

use super::subcommand::SubcommandHandler;
use crate::utils::paths::app_root;

#[derive(Parser, Debug)]
pub struct DeleteArgs {
    pub path: String,
}

pub struct DeleteHandler {
    path: String,
}

impl DeleteHandler {
    pub fn new(args: DeleteArgs) -> Self {
        DeleteHandler {
            path: args.path.to_owned(),
        }
    }
}

#[async_trait]
impl SubcommandHandler for DeleteHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let store_path = app_root().join(&self.path);
        fs::remove_dir_all(&store_path)?;

        Ok(None)
    }
}
