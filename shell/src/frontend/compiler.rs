use super::env::Environment;
use crate::frontend::{Arg, CompoundArg, ParseError, ShellCommandInterm};
use crate::ir::{CallCommand, Command, PipeCommand};
use std::collections::HashMap;

/// Compiler transforms inner shell command representation
/// into executable commands with given environment
pub struct Compiler {
    pub env: Environment,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn compile(&mut self, interm: Vec<ShellCommandInterm>) -> Result<PipeCommand, ParseError> {
        let mut commands = Vec::new();
        let env_copy = self.env.clone();
        for command_interm in interm {
            let arg_to_str = |arg: CompoundArg| {
                let transformed_parts: Vec<String> = arg
                    .inner
                    .iter()
                    .map(|p| match p {
                        Arg::String(str) => str.inner(&env_copy),
                        Arg::Var(name) => env_copy.get(name),
                        Arg::Number(n) => n.to_string(),
                    })
                    .collect();
                transformed_parts.join("")
            };
            match command_interm {
                ShellCommandInterm::Execute { name, args } => {
                    let name = arg_to_str(name);
                    let args: Vec<String> = args.into_iter().map(arg_to_str).collect();
                    let mut argv = vec![name.clone()];
                    argv.extend(args);
                    commands.push(CallCommand {
                        envs: HashMap::new(),
                        command: Command::from_name(&name),
                        argv,
                    })
                }
                ShellCommandInterm::Assign { name, value } => {
                    let value = value.map_or(String::from(""), arg_to_str);
                    self.env.set(&name, value);
                }
            }
        }

        Ok(PipeCommand { commands })
    }
}
