use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
}
