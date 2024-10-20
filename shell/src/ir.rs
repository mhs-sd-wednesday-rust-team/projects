use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Read, Write},
};

use crate::{
    backend::ExitStatus,
    builtins::{
        cat::CatCommand, echo::EchoCommand, exit::ExitCommand, pwd::PwdCommand, wc::WcCommand,
    },
};

#[derive(Debug)]
pub struct PipeCommand {
    pub commands: Vec<CallCommand>,
}

#[derive(Debug)]
pub struct CallCommand {
    pub envs: HashMap<String, String>,
    pub command: Command,
    pub argv: Vec<String>,
}

#[derive(Debug)]
pub enum Command {
    Call,
    Builtin(Box<dyn BuiltinCommand + Send>),
}

impl Command {
    pub fn from_name(name: &str) -> Self {
        match name {
            "cat" => Command::Builtin(Box::<CatCommand>::default()),
            "echo" => Command::Builtin(Box::<EchoCommand>::default()),
            "exit" => Command::Builtin(Box::<ExitCommand>::default()),
            "pwd" => Command::Builtin(Box::<PwdCommand>::default()),
            "wc" => Command::Builtin(Box::<WcCommand>::default()),
            _ => Command::Call,
        }
    }
}

pub trait BuiltinCommand: Debug {
    fn exec(
        &self,
        args: Vec<String>,
        stdin: &mut dyn Read,
        stderr: &mut dyn Write,
        stdout: &mut dyn Write,
    ) -> ExitStatus;
}
