use std::process::exit;
use clap::Parser;
use crate::backend::ExitStatus;

use super::BuiltinCommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    code: Option<i32>,
}

pub struct ExitCommand;

impl BuiltinCommand for ExitCommand {
    fn exec(args: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
        let args = Args::try_parse_from(args.into_iter())?;
        exit(args.code.unwrap_or_default())
    }
}