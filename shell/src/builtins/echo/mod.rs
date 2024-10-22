use std::error::Error;

use crate::ir::BuiltinCommand;
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
        stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        stdout: &mut dyn std::io::Write,
        piped_input: bool
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut content = if piped_input {
            let stdin_res = self.read_from_stdin(stdin)?;
            vec![stdin_res]
        } else {
            vec![]
        };
        let args = Args::try_parse_from(args.into_iter())?;
        content.extend(args.content);
        let mut output = content.join(" ");
        (!args.remove_trailing_newline).then(|| output += "\n");
        write!(stdout, "{}", output)?;
        Ok(())
    }
}
