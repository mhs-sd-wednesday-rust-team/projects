use crate::ir::BuiltinCommand;
use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<String>,
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
        stdin: &mut dyn Read,
        _stderr: &mut dyn Write,
        stdout: &mut dyn Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let args = Args::try_parse_from(args.into_iter())?;

        let mut buf = String::default();

        match args.file.as_deref() {
            Some("-") | None => {
                stdin.read_to_string(&mut buf)?;
            }
            Some(path) => {
                let mut file = File::open(path)?;
                file.read_to_string(&mut buf)?;
            }
        };

        write!(stdout, "{}", buf)?;
        Ok(())
    }

    fn tag(&self) -> &'static str {
        "cat"
    }
}
