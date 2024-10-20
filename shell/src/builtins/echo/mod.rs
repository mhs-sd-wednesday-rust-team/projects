use std::error::Error;

use crate::{backend::ExitStatus, ir::BuiltinCommand};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    content: Vec<String>,

    #[arg(short = 'n')]
    remove_trailing_newline: bool,
}

/// Implements the echo built-in command.
///
/// EchoCommand prints the provided arguments to the standard output.
#[derive(Default, Debug)]
pub struct EchoCommand;

impl BuiltinCommand for EchoCommand {
    fn exec(
        &self,
        args: Vec<String>,
        _stdin: &mut dyn std::io::Read,
        stderr: &mut dyn std::io::Write,
        stdout: &mut dyn std::io::Write,
    ) -> ExitStatus {
        let mut capture_stderr = |err: &dyn Error| {
            write!(stderr, "{}", err);
            1
        };

        let args = Args::try_parse_from(args.into_iter()).map_err(|err| capture_stderr(&err))?;
        let mut output = args.content.join(" ");
        (!args.remove_trailing_newline).then(|| output += "\n");
        write!(stdout, "{}", output);
        ExitStatus::Ok(())
    }
}
