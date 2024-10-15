use std::collections::HashMap;

pub struct Environment(pub HashMap<String, String>);

impl Environment {
    pub fn get(&self, key: &str) -> String {
        self.0.get(key).unwrap().clone()
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.0.insert(String::from(key), value);
    }
}
