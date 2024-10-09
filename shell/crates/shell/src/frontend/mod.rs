use std::io::Read;

use crate::ir::Command;

mod compiler;

pub struct Frontend {
    c: compiler::Compiler,
}

impl Frontend {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn parse<R: Read>(r: R) -> () {
        // -> impl Iterator<Item = Command> {
        unimplemented!()
    }
}
