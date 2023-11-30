use clap::Parser;
use colored::Colorize;
use safire::cli::*;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() {
    let result = run_async().await;

    match result {
        Ok(Some(msg)) => {
            println!("{}", msg);
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("Error: {}", err.to_string().red());
        }
    }
}

async fn run_async() -> Result<Option<String>, Box<dyn Error>> {
    let command = subcommand::Subcommand::parse();
    let command_handler: Box<dyn subcommand::SubcommandHandler> = get_handler(command);
    let result = command_handler.execute_async().await;

    result
}

fn get_handler(command: subcommand::Subcommand) -> Box<dyn subcommand::SubcommandHandler> {
    match command {
        subcommand::Subcommand::Add(args) => Box::new(add::AddHandler::new(args)),
        subcommand::Subcommand::Cp(args) => Box::new(copy::CopyHandler::new(args)),
        subcommand::Subcommand::Edit(args) => Box::new(edit::EditHandler::new(args)),
        subcommand::Subcommand::Find(args) => Box::new(find::FindHandler::new(args)),
        subcommand::Subcommand::Gen(args) => Box::new(generate::GenerateHandler::new(args)),
        subcommand::Subcommand::Grep(args) => Box::new(grep::GrepHandler::new(args)),
        subcommand::Subcommand::Init(args) => Box::new(init::InitHandler::new(args)),
        subcommand::Subcommand::Del(args) => Box::new(delete::DeleteHandler::new(args)),
    }
}
