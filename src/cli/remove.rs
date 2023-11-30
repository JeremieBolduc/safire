use async_trait::async_trait;
use clap::Parser;
use std::error::Error;
use std::fs;

use super::command_handler::CommandHandler;
use crate::utils::paths::get_app_path;

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub path: String,
}

pub struct RemoveHandler {
    path: String,
}

impl RemoveHandler {
    pub fn new(args: RemoveArgs) -> Self {
        RemoveHandler {
            path: args.path.to_owned(),
        }
    }
}

#[async_trait]
impl CommandHandler for RemoveHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let store_path = get_app_path().join(&self.path);
        fs::remove_dir_all(&store_path)?;

        Ok(None)
    }
}
