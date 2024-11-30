use std::{
    collections::HashMap,
    error::Error,
    fmt::Debug,
    io::{Read, Write},
};

use crate::builtins::{
    cat::CatCommand, cd::CdCommand, echo::EchoCommand, exit::ExitCommand, ls::LsCommand,
    pwd::PwdCommand, wc::WcCommand,
};

#[derive(Debug)]
pub struct PipeCommand {
    pub commands: Vec<CallCommand>,
}

#[derive(Debug, PartialEq)]
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
            "cd" => Command::Builtin(Box::<CdCommand>::default()),
            "ls" => Command::Builtin(Box::<LsCommand>::default()),
            _ => Command::Call,
        }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Command::Call, Command::Call) => true,
            (Command::Builtin(cmd_self), Command::Builtin(cmd_other)) => {
                cmd_self.tag() == cmd_other.tag()
            }
            _ => false,
        }
    }
}

pub trait BuiltinCommand: Debug {
    fn tag(&self) -> &'static str;

    fn exec(
        &self,
        args: Vec<String>,
        stdin: &mut dyn Read,
        stderr: &mut dyn Write,
        stdout: &mut dyn Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>>;
}
