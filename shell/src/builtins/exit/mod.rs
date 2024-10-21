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
        _stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        _stdout: &mut dyn std::io::Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let args = Args::try_parse_from(args.into_iter()).unwrap_or_default();
        exit(args.code.unwrap_or_default())
    }
}
