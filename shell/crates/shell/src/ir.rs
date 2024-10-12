use std::collections::HashMap;

pub enum Command {
    PipeCommand(PipeCommand),
    CallCommand(CallCommand),
    ExitCommand,
}

pub struct PipeCommand {
    pub commands: Vec<Box<Command>>,
}

pub struct CallCommand {
    pub envs: HashMap<String, String>,
    pub argv: Vec<String>,
}
