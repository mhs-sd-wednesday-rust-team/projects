use crate::backend::ExitStatus;
use clap::Parser;

use super::BuiltinCommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    content: Vec<String>,

    #[arg(short = 'n')]
    remove_trailing_newline: bool,
}

pub struct EchoCommand;

impl BuiltinCommand for EchoCommand {
    fn exec(args: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
        let args = Args::try_parse_from(args.into_iter())?;
        let mut output = args.content.join(" ");
        (!args.remove_trailing_newline).then(|| output += "\n");
        print!("{output}");
        Ok(ExitStatus::default())
    }
}
