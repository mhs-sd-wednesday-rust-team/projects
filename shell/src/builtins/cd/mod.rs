use std::{env::set_current_dir, error::Error};

use crate::ir::BuiltinCommand;
use clap::Parser;
use home::home_dir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

#[derive(Default, Debug)]
pub struct CdCommand;

// Implementation of cd builtin-command
//
// Changes the current working directory to the provided path
impl BuiltinCommand for CdCommand {
    fn exec(
        &self,
        args: Vec<String>,
        _stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        _stdout: &mut dyn std::io::Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let p: String;
        if args.len() == 1 {
            p = home_dir()
                .ok_or("failed to get home_dir")?
                .as_path()
                .to_str()
                .unwrap()
                .to_string();
        } else {
            p = Args::try_parse_from(args.into_iter())?.path;
        }

        set_current_dir(p)?;
        Ok(())
    }

    fn tag(&self) -> &'static str {
        "cd"
    }
}
