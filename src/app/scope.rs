use std::collections::HashMap;

pub struct Scope {
    dict: HashMap<String, String>,
}

impl Scope {
    pub fn new() -> Self {
        Self { dict: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.dict.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.dict.insert(key.into(), value.into());
    }
}