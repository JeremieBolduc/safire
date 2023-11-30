use async_trait::async_trait;
use std::error::Error;

use super::command_handler::CommandHandler;
use crate::utils::{directories::find_directories, paths::get_app_path};

pub struct FindHandler {
    query: String,
}

impl FindHandler {
    pub fn new(search_string: &str) -> Self {
        FindHandler {
            query: search_string.to_owned(),
        }
    }
}

#[async_trait]
impl CommandHandler for FindHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>> {
        let find_result = find_directories(&self.query, &get_app_path());

        match find_result {
            Ok(entries) => {
                for entry in entries {
                    println!("{}", entry.display());
                }
                Ok(None)
            }
            Err(err) => {
                eprintln!("Error getting the directories: {}", err);
                Err(err)
            }
        }
    }
}
