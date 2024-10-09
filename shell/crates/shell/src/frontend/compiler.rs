use crate::ir::Command;

pub struct Compiler;

impl Compiler {
    pub fn compile<I: Iterator<Item = ()>>(ast: I) -> () {
        // -> impl Iterator<Item = Command> {
        unimplemented!()
    }
}
