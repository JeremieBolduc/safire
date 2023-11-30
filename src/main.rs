use clap::Parser;
use std::error::Error;
use tokio;

pub mod cli;
pub mod data;
pub mod utils;

use crate::cli::*;

#[tokio::main]
async fn main() {
    let result = run_async().await;

    match result {
        Ok(Some(msg)) => {
            println!("{}", msg);
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}

async fn run_async() -> Result<Option<String>, Box<dyn Error>> {
    let command = command::Command::parse();
    let command_handler: Box<dyn command_handler::CommandHandler> = get_handler(command);
    let result = command_handler.execute_async().await;

    result
}

fn get_handler(command: command::Command) -> Box<dyn command_handler::CommandHandler> {
    match command {
        command::Command::Cp(args) => Box::new(copy::CopyHandler::new(args)),
        command::Command::Edit(args) => Box::new(edit::EditHandler::new(args)),
        command::Command::Find { query } => Box::new(find::FindHandler::new(&query)),
        command::Command::Gen(args) => Box::new(generate::GenerateHandler::new(args)),
        command::Command::Grep(args) => Box::new(grep::GrepHandler::new(args)),
        command::Command::Init(args) => Box::new(init::InitHandler::new(args)),
        command::Command::Insert(args) => Box::new(insert::InsertHandler::new(args)),
        command::Command::Rm(args) => Box::new(remove::RemoveHandler::new(args)),
    }
}
