use crate::backend::ExitStatus;
use clap::Parser;
use std::{fs::File, io::Read};

use super::BuiltinCommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
}

pub struct CatCommand;

impl BuiltinCommand for CatCommand {
    fn exec(args: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
        let args = Args::try_parse_from(args.into_iter())?;
        let mut file = File::open(args.file)?;

        let mut buf = String::default();
        file.read_to_string(&mut buf)?;

        print!("{buf}");

        Ok(ExitStatus::default())
    }
}
