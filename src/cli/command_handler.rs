use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait CommandHandler {
    async fn execute_async(&self) -> Result<Option<String>, Box<dyn Error>>;
}
