use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub password: String,
    pub meta_data: Value,
}

impl Store {
    pub fn new(password: &str, meta_data: Option<Value>) -> Self {
        Store {
            password: password.to_owned(),
            meta_data: meta_data.unwrap_or_default(),
        }
    }
}
