use std::io::Read;
use std::fmt::{Debug, Display};
use conch_parser::ast::{Command, ListableCommand, Parameter, PipeableCommand, RedirectOrCmdWord, RedirectOrEnvVar, ShellWord, SimpleWord, TopLevelWord, Word};
use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;

mod compiler;
mod env;

pub struct Frontend {}

#[derive(Debug, Clone, PartialEq)]
pub enum StringArg {
    DoubleQuoted(String),
    SingleQuoted(String),
    Simple(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    String(StringArg),
    Var(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellCommand {
    Execute {
        name: Arg,
        args: Vec<Arg>
    },
    Assign {
        name: String,
        value: Option<Arg>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipedCommands {
    commands: Vec<ShellCommand>
}

fn parse_top_level_word<T: Debug + Display>(word: TopLevelWord<T>) -> Arg {
    let ShellWord::Single(word) = word.0 else {
        panic!("Concated words are unsupported.");
    };

    let parse_simple_word = |sw| {
        match sw {
            SimpleWord::Literal(l) => {
                let l_strint = format!("{l}");
                let parsed_number = l_strint.parse::<f64>();
                if let Ok(parsed_number) = parsed_number {
                    Arg::Number(parsed_number)
                } else {
                    Arg::String(StringArg::Simple(l_strint))
                }
            }
            SimpleWord::Escaped(_) => {
                // escaped token
                todo!()
            }
            SimpleWord::Param(p) => {
                let Parameter::Var(var) = p else {
                    panic!("Only var parameters are supported.")
                };
                Arg::Var(format!("{var}"))
            }
            SimpleWord::Subst(_) => {
                // param substitution
                todo!()
            }
            _ => panic!("Unsupported simple.")
        }
    };

    let arg = match word {
        Word::Simple(simple) => {
            parse_simple_word(simple)
        }
        Word::DoubleQuoted(dq) => {
            // TODO: Why is it a vector?
            let parsed: Vec<Arg> = dq.into_iter()
                .map(|sq| parse_simple_word(sq))
                .collect();
            parsed.first().unwrap().clone()
        }
        Word::SingleQuoted(sq) => {
            Arg::String(StringArg::SingleQuoted(format!("{sq}")))
        }
    };
    println!("{arg:?}");
    arg
}

impl Frontend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, input: &str) -> Vec<ShellCommand> {
        let lex = Lexer::new(input.chars());
        let parser = DefaultParser::new(lex);
        let parse_res = parser.into_iter().next().expect("Expected parser result.");

        let Ok(parse_res) = parse_res else {
            panic!("Failed to parse input.")
        };
        let command = parse_res.0;
        let Command::List(commands_list) = command else {
            panic!("Async commands are unsupported.")
        };
        // We don't support "||" and "&&" so that `commands_list.second` is empty.
        let first_command = commands_list.first;
        let commands_vec = match first_command {
            ListableCommand::Pipe(_, commands) => {
                commands
            }
            ListableCommand::Single(command) => {
                vec![command]
            }
        };
        let mut piped_commands = Vec::new();
        for command in commands_vec {
            println!("New pipe command:");
            let PipeableCommand::Simple(simple_command) = command else {
                panic!("Only simple pipeable commands are supported.")
            };

            if !simple_command.redirects_or_env_vars.is_empty() {
                // Case of variable assign.
                let command_values = simple_command.redirects_or_env_vars;
                let assign = command_values.first().expect("var assign expected.").to_owned();
                let RedirectOrEnvVar::EnvVar(name, value) = assign else {
                    panic!("Expected var declaration.")
                };
                let value = value.map(|v| parse_top_level_word(v));
                piped_commands.push(ShellCommand::Assign {
                    name,
                    value
                });
                continue
            }

            let command_values = simple_command.redirects_or_cmd_words;

            let values_parsed: Vec<Arg> = command_values
                .into_iter()
                .map(|value| {
                    let RedirectOrCmdWord::CmdWord(toplevel_word) = value else {
                        panic!("Redirection is unsupported.")
                    };
                    parse_top_level_word(toplevel_word)
                })
                .collect();
            let (name, args) = values_parsed.split_first().expect("expected command with args.");
            piped_commands.push(ShellCommand::Execute {
                name: name.clone(),
                args: args.clone().to_vec()
            })
        }

        for command in &piped_commands {
            println!("{command:?}")
        }

        piped_commands
    }
}
