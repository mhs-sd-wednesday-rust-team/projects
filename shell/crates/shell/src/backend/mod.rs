use std::{
    error::Error,
    io::{Read, Write},
};

use crate::ir::Command;

pub struct Backend;

impl Backend {
    pub fn exec<W, R>(&mut self, command: Command, r: R, w: W) -> Result<(), Box<dyn Error>>
    where
        R: Read,
        W: Write,
    {
        unimplemented!()
    }
}
