use std::error::Error;

pub trait CommandHandler {
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>>;
}
