use std::collections::HashMap;

#[derive(Debug)]
pub struct PipeCommand {
    pub commands: Vec<CallCommand>,
}

#[derive(Debug)]
pub struct CallCommand {
    pub envs: HashMap<String, String>,
    pub argv: Vec<String>,
}
