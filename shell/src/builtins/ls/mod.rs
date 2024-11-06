use std::{error::Error, fs, path};

use crate::ir::BuiltinCommand;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

#[derive(Default, Debug)]
pub struct LsCommand;

// Implementation of ls builtin-command
//
// Lists the contents of the given path or the current directory if no path is given.
impl BuiltinCommand for LsCommand {
    fn exec(
        &self,
        args: Vec<String>,
        _stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        stdout: &mut dyn std::io::Write,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let p: path::PathBuf;
        p = if args.len() == 1 {
            std::env::current_dir()?
        } else {
            let args = Args::try_parse_from(args.into_iter())?;
            path::Path::new(&args.path).to_path_buf()
        };

        let entries = fs::read_dir(p)?;

        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name();
            writeln!(stdout, "{}", file_name.to_string_lossy())?;
        }
        Ok(())
    }

    fn tag(&self) -> &'static str {
        "ls"
    }
}
