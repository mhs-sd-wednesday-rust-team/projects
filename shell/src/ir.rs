use std::collections::HashMap;

#[derive(Debug)]
pub enum Command {
    CallCommand(CallCommand),
    ExitCommand(Option<i32>),
}

#[derive(Debug)]
pub struct PipeCommand {
    pub commands: Vec<Command>,
}

#[derive(Debug)]
pub struct CallCommand {
    pub envs: HashMap<String, String>,
    pub argv: Vec<String>,
}

