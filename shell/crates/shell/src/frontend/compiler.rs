use std::collections::HashMap;
use crate::ir::Command;

use super::env::Environment;

pub struct Compiler {
    pub env: Environment,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            env: Environment(HashMap::new())
        }
    }

    pub fn compile<I: Iterator<Item = ()>>(ast: I) -> () {
        unimplemented!()
    }
}
