use std::collections::HashMap;

pub struct Scope {
    dict: HashMap<String, String>,
}

impl Scope {
    pub fn new() -> Self {
        Self { dict: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> &str {
        self.dict.get(key).map(String::as_str).unwrap_or("")
    }

    pub fn set(&mut self, key: &str, value: &str) {
        if value.is_empty() {
            self.dict.remove(key);
        } else if let Some(existing) = self.dict.get_mut(key) {
            existing.clear();
            existing.push_str(value);
        } else {
            self.dict.insert(String::from(key), String::from(value));
        }
    }
}