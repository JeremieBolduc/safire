use clap::Parser;
use std::error::Error;

pub mod cli;
pub mod data;
pub mod utils;

use crate::cli::*;

fn main() {
    let result = run();

    match result {
        Ok(Some(msg)) => {
            println!("{}", msg);
            // msg;
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}

fn run() -> Result<Option<String>, Box<dyn Error>> {
    let command = command::Command::parse();
    let command_handler: Box<dyn command_handler::CommandHandler> = get_handler(command);
    let result = command_handler.execute();

    result
}

fn get_handler(command: command::Command) -> Box<dyn command_handler::CommandHandler> {
    match command {
        command::Command::Edit(args) => Box::new(edit::EditHandler::new(args)),
        command::Command::Insert(args) => Box::new(insert::InsertHandler::new(args)),
        command::Command::Cp(args) => Box::new(copy::CopyHandler::new(args)),
        command::Command::Gen(args) => Box::new(generate::GenerateHandler::new(args)),
        command::Command::Find { query } => Box::new(find::FindHandler::new(&query)),
        command::Command::Rm(args) => Box::new(remove::RemoveHandler::new(args)),
    }
}
