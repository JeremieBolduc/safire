use std::error::Error;

use crate::commands::command_handler::CommandHandler;
use crate::utils::find::find_directories;

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

impl CommandHandler for FindHandler {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let find_result = find_directories(&self.query);

        match find_result {
            Ok(entries) => {
                for entry in entries {
                    println!("{}", entry.display());
                }
                Ok(())
            }
            Err(err) => {
                eprintln!("Error getting the directories: {}", err);
                Err(err)
            }
        }
    }
}
