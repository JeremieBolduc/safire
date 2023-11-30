use super::*;
use clap::Parser;

/// A simple local password store
#[derive(Parser, Debug)]
#[clap(
    author = "Jeremie Bolduc",
    version,
    about = "A simple password management tool"
)]
pub enum Command {
    /// Opens a text editor to the text file that contains the store password and meta-data
    Edit(edit::EditArgs),
    Init(init::InitArgs),
    /// Adds a new password for the given key
    Insert(insert::InsertArgs),
    /// Copies a password to the clipboard
    Cp(copy::CopyArgs),
    /// Generates a password for the given key
    Gen(generate::GenerateArgs),

    Grep(grep::GrepArgs),
    /// Attempts to find stores that match a query string
    Find {
        query: String,
    },
    /// Removes a store
    Rm(remove::RemoveArgs),
}
