use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment(HashMap<String, String>);

impl Environment {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> String {
        self.0.get(key).unwrap_or(&String::from("")).clone()
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.0.insert(String::from(key), value);
    }
}
