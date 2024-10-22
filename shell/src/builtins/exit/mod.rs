use crate::ir::BuiltinCommand;
use clap::Parser;
use std::{error::Error, process::exit};

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
struct Args {
    code: Option<i32>,
}

/// Implements the exit built-in command.
///
/// ExitCommand terminates the shell optional exit status code.
/// If no code is provided, it defaults to exiting with status 0.
#[derive(Default, Debug)]
pub struct ExitCommand;

impl BuiltinCommand for ExitCommand {
    fn exec(
        &self,
        args: Vec<String>,
        stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        _stdout: &mut dyn std::io::Write,
        piped_input: bool
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let code = if piped_input {
            let stdin_res = self.read_from_stdin(stdin)?;
            let parse_res = stdin_res.parse::<i32>();
            match parse_res {
                Ok(code) => code,
                Err(e) => return Err(e.into())
            }
        } else {
            let args = Args::try_parse_from(args).unwrap_or_default();
            args.code.unwrap_or_default()
        };
        exit(code)
    }
}
