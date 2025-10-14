use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    name: String,
    #[serde(flatten)]
    data: HashMap<String, Value>,
    #[serde(skip)]
    modifiable: bool,
}

impl Event {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: HashMap::new(),
            modifiable: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if !self.modifiable { return self; }

        let value = serde_json::to_value(value).unwrap_or(Value::Null);
        self.data.insert(key.into(), value);
        self
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.data.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn freeze(mut self) -> Self {
        self.modifiable = false;
        self
    }

    pub fn is_frozen(&self) -> bool {
        !self.modifiable
    }
}

