use clap::Parser;
use std::error::Error;
use std::process::Command;

use crate::commands::command_handler::CommandHandler;
use crate::utils::constants::APP_NAME;

#[derive(Parser, Debug)]
pub struct EditArgs {
    path: String,
    #[clap(short, long, default_value = "nano")]
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
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.json", self.path.replace("/", "-"));
        let home_dir = dirs::home_dir().ok_or("Unable to determine home directory")?;
        let file_path = home_dir.join(APP_NAME).join(&self.path).join(file_name);

        let status = Command::new(self.editor.as_str())
            .arg(&file_path)
            .status()?;

        println!("Editing store {} with {}", &self.path, self.editor);

        if status.success() {
            Ok(())
        } else {
            Err("Editor process exited with an error".into())
        }
    }
}
