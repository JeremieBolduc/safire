use clap::Parser;
use std::error::Error;
use std::process::Command;

use super::command_handler::CommandHandler;
use crate::utils::paths::get_app_path;

#[derive(Parser, Debug)]
pub struct EditArgs {
    path: String,
    #[clap(short, long, default_value = "vi")]
    pub editor: String,
}

pub struct EditHandler {
    path: String,
    editor: String,
}

impl EditHandler {
    pub fn new(args: EditArgs) -> Self {
        EditHandler {
            path: args.path.to_owned(),
            editor: args.editor.to_owned(),
        }
    }
}

impl CommandHandler for EditHandler {
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>> {
        let file_name = format!("{}.json", self.path.replace("/", "-"));
        let app_path = get_app_path();
        let store_path = app_path.join(&self.path);
        let file_path = store_path.join(&file_name);

        if !store_path.exists() {
            return Ok(Some(format!("Could not find store for {}", self.path)));
        }

        if !file_path.exists() {
            return Ok(Some(format!(
                "Could not find the data file for {}",
                self.path
            )));
        }

        let status = Command::new(self.editor.as_str())
            .arg(&file_path)
            .status()?;

        println!("Editing store {} with {}", &self.path, self.editor);

        if status.success() {
            Ok(None)
        } else {
            Err("Editor process exited with an error".into())
        }
    }
}
