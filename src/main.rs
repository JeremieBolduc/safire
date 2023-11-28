pub mod commands;
pub mod entities;
pub mod utils;

use clap::Parser;
use std::error::Error;

use crate::commands::*;

/// A simple local password store
#[derive(Parser, Debug)]
#[clap(
    author = "Jeremie Bolduc",
    version,
    about = "A simple password management tool"
)]
enum Command {
    /// Opens a text editor to the text file that contains the store password and meta-data
    Edit(edit::EditArgs),
    /// Adds a new password for the given key
    Insert(insert::InsertArgs),
    /// Copies a password to the clipboard
    Cp(copy::CopyArgs),
    /// Generates a password for the given key
    Gen(generate::GenerateArgs),
    /// Attempts to find stores that match a query string
    Find { query: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let command = Command::parse();
    let command_handler: Box<dyn command_handler::CommandHandler> = get_handler(command);
    let result = command_handler.execute();

    return result;
}

fn get_handler(command: Command) -> Box<dyn command_handler::CommandHandler> {
    match command {
        Command::Edit(args) => Box::new(edit::EditHandler::new(args)),
        Command::Insert(args) => Box::new(insert::InsertHandler::new(args)),
        Command::Cp(args) => Box::new(copy::CopyHandler::new(args)),
        Command::Gen(args) => Box::new(generate::GenerateHandler::new(args)),
        Command::Find { query } => Box::new(find::FindHandler::new(&query)),
    }
}
