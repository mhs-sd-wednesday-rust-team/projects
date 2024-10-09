use std::collections::HashMap;

pub struct Environment(HashMap<String, String>);

impl Environment {
    pub fn get(&self, key: &str) -> String {
        unimplemented!()
    }

    pub fn set(&mut self, key: &str, value: String) {
        unimplemented!()
    }
}
