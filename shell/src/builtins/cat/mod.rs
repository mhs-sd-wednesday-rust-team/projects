use crate::{backend::ExitStatus, ir::BuiltinCommand};
use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
}

/// Implements the cat built-in command.
///
/// CatCommand reads and prints the contents of a specified file to standard output.
#[derive(Default, Debug)]
pub struct CatCommand;

impl BuiltinCommand for CatCommand {
    fn exec(
        &self,
        args: Vec<String>,
        _stdin: &mut dyn Read,
        stderr: &mut dyn Write,
        stdout: &mut dyn Write,
    ) -> ExitStatus {
        let mut capture_stderr = |err: &dyn Error| {
            write!(stderr, "{}", err);
            1
        };

        let args = Args::try_parse_from(args.into_iter()).map_err(|err| capture_stderr(&err))?;
        let mut file = File::open(args.file).map_err(|err| capture_stderr(&err))?;

        let mut buf = String::default();
        file.read_to_string(&mut buf)
            .map_err(|err| capture_stderr(&err))?;

        write!(stdout, "{}", buf);

        ExitStatus::Ok(())
    }
}
