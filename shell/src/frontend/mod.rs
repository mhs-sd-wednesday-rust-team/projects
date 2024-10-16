use crate::ir::PipeCommand;
use conch_parser::ast;
use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use std::fmt::{Debug, Display};

pub mod compiler;
mod env;

#[derive(Debug, Clone, PartialEq)]
pub enum StringArg {
    #[allow(dead_code)]
    DoubleQuoted(String),
    SingleQuoted(String),
    Simple(String),
}

impl StringArg {
    fn inner(&self) -> String {
        match self {
            StringArg::DoubleQuoted(inner)
            | StringArg::SingleQuoted(inner)
            | StringArg::Simple(inner) => inner.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    String(StringArg),
    Var(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellCommandInterm {
    Execute {
        name: CompoundArg,
        args: Vec<CompoundArg>,
    },
    Assign {
        name: String,
        value: Option<CompoundArg>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        ParseError {
            message: String::from(value),
        }
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError { message: value }
    }
}

/// Helper struct to denote result of "$x$x" input resulting in a vec of [Arg::Var, Arg::Var];
/// Should later be concatenated into a single string.
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundArg {
    pub inner: Vec<Arg>,
}

impl CompoundArg {
    fn new(inner: Vec<Arg>) -> Self {
        Self { inner }
    }
}

fn parse_top_level_word<T: Debug + Display>(
    word: ast::TopLevelWord<T>,
) -> Result<CompoundArg, ParseError> {
    let words = match word.0 {
        ast::ShellWord::Single(s) => {
            vec![s]
        }
        ast::ShellWord::Concat(s_v) => s_v,
    };
    let parse_simple_word = |sw| match sw {
        ast::SimpleWord::Literal(l) => {
            let l_strint = format!("{l}");
            let parsed_number = l_strint.parse::<f64>();
            let arg = if let Ok(parsed_number) = parsed_number {
                Arg::Number(parsed_number)
            } else {
                Arg::String(StringArg::Simple(l_strint))
            };
            Ok(arg)
        }
        ast::SimpleWord::Param(p) => {
            let ast::Parameter::Var(var) = p else {
                return Err("Only var parameters of $var view are supported.".into());
            };
            Ok(Arg::Var(format!("{var}")))
        }
        _ => Err(ParseError::from(format!("Unsupported token: {sw:?}."))),
    };

    let mut processed_args = Vec::new();
    for word in words {
        let arg = match word {
            ast::Word::Simple(simple) => parse_simple_word(simple)?,
            ast::Word::DoubleQuoted(dq) => {
                let parsed: Result<Vec<Arg>, ParseError> =
                    dq.into_iter().map(parse_simple_word).collect();
                if let Some(first) = parsed?.first() {
                    first.clone()
                } else {
                    return Err("Doublde quoted string is empty".into());
                }
            }
            ast::Word::SingleQuoted(sq) => Arg::String(StringArg::SingleQuoted(format!("{sq}"))),
        };
        processed_args.push(arg)
    }
    Ok(CompoundArg::new(processed_args))
}

pub fn parse_intermediate(input: &str) -> Result<Vec<ShellCommandInterm>, ParseError> {
    let lex = Lexer::new(input.chars());
    let parser = DefaultParser::new(lex);
    let parse_res = parser.into_iter().next();
    let Some(parse_res) = parse_res else {
        return Err("Unable to parse (possibly empty) input".into());
    };

    let Ok(parse_res) = parse_res else {
        // Fatal error input
        return Err(format!("{parse_res:?}").into());
    };
    let command = parse_res.0;
    let ast::Command::List(commands_list) = command else {
        return Err("Async commands with & are unsupported.".into());
    };
    // We don't support "||" and "&&" so that `commands_list.second` is empty.
    let first_command = commands_list.first;
    let commands_vec = match first_command {
        ast::ListableCommand::Pipe(_, commands) => commands,
        ast::ListableCommand::Single(command) => {
            vec![command]
        }
    };
    let mut piped_commands = Vec::new();
    for command in commands_vec {
        let ast::PipeableCommand::Simple(simple_command) = command else {
            return Err("Functions and fd redirection are not supported.".into());
        };

        if !simple_command.redirects_or_env_vars.is_empty() {
            // Case of variable assign.
            let command_values = simple_command.redirects_or_env_vars;
            let assign = command_values
                .first()
                .expect("var assign expected.")
                .to_owned();
            let ast::RedirectOrEnvVar::EnvVar(name, value) = assign else {
                return Err("Expected variable declaration. Redirection is not supported.".into());
            };
            let value = value.map(parse_top_level_word);
            let value = match value {
                None => None,
                Some(value) => Some(value?),
            };
            piped_commands.push(ShellCommandInterm::Assign { name, value });
            continue;
        }

        let command_values = simple_command.redirects_or_cmd_words;

        let values_parsed: Result<Vec<CompoundArg>, ParseError> = command_values
            .into_iter()
            .map(|value| {
                let ast::RedirectOrCmdWord::CmdWord(toplevel_word) = value else {
                    return Err(ParseError::from(
                        "Expected command word token. Redirection is not supported.",
                    ));
                };
                parse_top_level_word(toplevel_word)
            })
            .collect();
        let values_parsed = values_parsed?;
        let (name, args) = values_parsed
            .split_first()
            .expect("expected command with args.");
        piped_commands.push(ShellCommandInterm::Execute {
            name: name.clone(),
            args: args.to_vec(),
        })
    }

    Ok(piped_commands)
}

pub struct Frontend {
    c: compiler::Compiler,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            c: compiler::Compiler::new(),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<PipeCommand, ParseError> {
        let interm = parse_intermediate(input)?;
        self.c.compile(interm)
    }
}
