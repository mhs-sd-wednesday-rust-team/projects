use crate::ir::PipeCommand;
use conch_parser::ast;
use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use env::Environment;
use std::fmt::{Debug, Display};

pub mod compiler;
mod env;

#[derive(Debug, Clone, PartialEq)]
pub enum StringArg {
    #[allow(dead_code)]
    DoubleQuoted(Vec<Arg>),
    SingleQuoted(String),
    Simple(String),
}

impl StringArg {
    fn inner(&self, env: &Environment) -> String {
        match self {
            StringArg::DoubleQuoted(inner) => {
                let strings_list: Vec<String> = inner
                    .iter()
                    .map(|a| match a {
                        Arg::String(string_arg) => match string_arg {
                            StringArg::DoubleQuoted(_) => {
                                panic!("Recursive DoubleQuoted string met.")
                            }
                            StringArg::SingleQuoted(inner) | StringArg::Simple(inner) => {
                                inner.clone()
                            }
                        },
                        Arg::Var(name) => env.get(name),
                        Arg::Number(n) => n.to_string(),
                    })
                    .collect();
                strings_list.join("")
            }
            StringArg::SingleQuoted(inner) | StringArg::Simple(inner) => inner.clone(),
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
                Arg::String(StringArg::DoubleQuoted(parsed?))
            }
            ast::Word::SingleQuoted(sq) => Arg::String(StringArg::SingleQuoted(format!("{sq}"))),
        };
        processed_args.push(arg)
    }
    Ok(CompoundArg::new(processed_args))
}

/// Parses the intermediate representation of a shell command from the input string.
///
/// It currently supports only simple single-command inputs.
///
/// # Errors
///
/// This function will return errors for unsupported syntax such as asynchronous commands,
/// logical operators, functions, and file descriptor redirection. It also returns errors
/// when parsing fails.
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

/// The frontend transforms commands from string representation
/// to an executable form using compiler
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        frontend::{env::Environment, Arg, StringArg},
        ir::CallCommand,
    };

    use super::{parse_intermediate, Frontend, ShellCommandInterm};

    #[test]
    fn test_parse_intermidiate() {
        let input = r#"x=1 | y= | com1 2 arg2 | com2 'arg3' "arg4" | com3 "a$x$y" $x$y "#;
        let interm = parse_intermediate(input).unwrap();
        let mut interm_iter = interm.into_iter();

        let ShellCommandInterm::Assign { name, value } = interm_iter.next().unwrap() else {
            panic!("Expected Assign")
        };
        assert_eq!(name, "x");
        assert_eq!(*value.unwrap().inner.first().unwrap(), Arg::Number(1.0));

        let ShellCommandInterm::Assign { name, value } = interm_iter.next().unwrap() else {
            panic!("Expected Assign")
        };
        assert_eq!(name, "y");
        assert_eq!(value, None);

        let ShellCommandInterm::Execute { name, args } = interm_iter.next().unwrap() else {
            panic!("Expected Execute")
        };
        assert_eq!(
            *name.inner.first().unwrap(),
            Arg::String(StringArg::Simple(String::from("com1")))
        );
        let mut args_iter = args.into_iter();
        assert_eq!(args_iter.next().unwrap().inner, vec![Arg::Number(2.0)]);
        assert_eq!(
            args_iter.next().unwrap().inner,
            vec![Arg::String(StringArg::Simple(String::from("arg2")))]
        );

        let ShellCommandInterm::Execute { name, args } = interm_iter.next().unwrap() else {
            panic!("Expected Execute")
        };
        assert_eq!(
            *name.inner.first().unwrap(),
            Arg::String(StringArg::Simple(String::from("com2")))
        );
        let mut args_iter = args.into_iter();
        assert_eq!(
            args_iter.next().unwrap().inner,
            vec![Arg::String(StringArg::SingleQuoted(String::from("arg3"))),]
        );
        assert_eq!(
            args_iter.next().unwrap().inner,
            vec![Arg::String(StringArg::DoubleQuoted(vec![Arg::String(
                StringArg::Simple(String::from("arg4"))
            )]))]
        );

        let ShellCommandInterm::Execute { name, args } = interm_iter.next().unwrap() else {
            panic!("Expected Execute")
        };

        assert_eq!(
            *name.inner.first().unwrap(),
            Arg::String(StringArg::Simple(String::from("com3")))
        );
        let mut args_iter = args.into_iter();
        assert_eq!(
            args_iter.next().unwrap().inner,
            vec![Arg::String(StringArg::DoubleQuoted(vec![
                Arg::String(StringArg::Simple(String::from("a"))),
                Arg::Var(String::from("x")),
                Arg::Var(String::from("y"))
            ]))]
        );
        assert_eq!(
            args_iter.next().unwrap().inner,
            vec![Arg::Var(String::from("x")), Arg::Var(String::from("y"))]
        );
    }

    #[test]
    fn test_parse_full_no_vars() {
        let mut front = Frontend::new();
        let input = r#"echo 1 '2' "3" | cat foo bar"#;
        let mut commands = front.parse(&input).unwrap().commands.into_iter();
        assert_eq!(
            commands.next().unwrap(),
            CallCommand {
                envs: HashMap::new(),
                argv: vec![
                    String::from("echo"),
                    String::from("1"),
                    String::from("2"),
                    String::from("3")
                ]
            }
        );
        assert_eq!(
            commands.next().unwrap(),
            CallCommand {
                envs: HashMap::new(),
                argv: vec![
                    String::from("cat"),
                    String::from("foo"),
                    String::from("bar")
                ]
            }
        );
    }

    #[test]
    fn test_parse_full_assign_change_state() {
        let mut front = Frontend::new();
        let input = r#"x=1"#;
        front.parse(&input).unwrap();
        let mut expected_env = Environment::new();
        expected_env.set("x", String::from("1"));

        assert_eq!(front.c.env, expected_env);
    }

    #[test]
    fn test_parse_full_assign_is_not_visible() {
        let mut front = Frontend::new();
        let input = r#"x=1 | echo $x"#;
        let mut commands = front.parse(&input).unwrap().commands.into_iter();
        assert_eq!(
            commands.next().unwrap(),
            CallCommand {
                envs: HashMap::new(),
                argv: vec![String::from("echo"), String::from("")]
            }
        );
    }

    #[test]
    fn test_parse_full_assign_is_visible() {
        let mut front = Frontend::new();
        let input = r#"x=1"#;
        front.parse(&input).unwrap();

        let input = r#"echo $x"#;
        let mut commands = front.parse(&input).unwrap().commands.into_iter();
        assert_eq!(
            commands.next().unwrap(),
            CallCommand {
                envs: HashMap::new(),
                argv: vec![String::from("echo"), String::from("1")]
            }
        );
    }
}
