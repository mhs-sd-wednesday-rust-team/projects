use crate::ir::Command;

use super::env::Environment;

pub struct Compiler {
    env: Environment,
}

impl Compiler {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn compile<I: Iterator<Item = ()>>(ast: I) -> () {
        // -> impl Iterator<Item = Command> {
        unimplemented!()
    }
}
