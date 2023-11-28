use std::error::Error;

pub trait CommandHandler {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
