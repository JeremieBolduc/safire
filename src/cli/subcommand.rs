use super::*;
use async_trait::async_trait;
use clap::Parser;
use std::error::Error;

/// A simple local password management tool
#[derive(Parser, Debug)]
#[clap(
    author = "Jeremie Bolduc",
    version,
    about = "A simple password management tool"
)]
pub enum Subcommand {
    /// Adds a new password store
    Add(add::AddArgs),
    /// Copies a password to the clipboard
    Cp(copy::CopyArgs),
    /// Deletes a store
    Del(delete::DeleteArgs),
    /// Opens a text editor to the text file that contains the store password and meta-data
    Edit(edit::EditArgs),
    /// Attempts to find stores that match a query string
    Find(find::FindArgs),
    /// Generates a password for the given key
    Gen(generate::GenerateArgs),
    /// Runs a Regular Expression search in the files
    Grep(grep::GrepArgs),
    Init(init::InitArgs),
}

#[async_trait]
pub trait SubcommandHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>>;
}
