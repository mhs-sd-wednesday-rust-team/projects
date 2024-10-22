use crate::ir::BuiltinCommand;
use std::{env::current_dir, error::Error};

/// Implements the pwd built-in command.
///
/// PwdCommand retrieves and prints the current working directory to the standard output.
#[derive(Default, Debug)]
pub struct PwdCommand;

impl BuiltinCommand for PwdCommand {
    fn exec(
        &self,
        _args: Vec<String>,
        _stdin: &mut dyn std::io::Read,
        _stderr: &mut dyn std::io::Write,
        stdout: &mut dyn std::io::Write,
        _piped_input: bool
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let path = current_dir()?;
        writeln!(stdout, "{}", path.display())?;
        Ok(())
    }
}
