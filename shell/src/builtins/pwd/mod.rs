use crate::{backend::ExitStatus, ir::BuiltinCommand};
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
        stderr: &mut dyn std::io::Write,
        stdout: &mut dyn std::io::Write,
    ) -> ExitStatus {
        let mut capture_stderr = |err: &dyn Error| {
            write!(stderr, "{}", err);
            1
        };

        let path = current_dir().map_err(|err| capture_stderr(&err))?;
        write!(stdout, "{}", path.display());
        ExitStatus::Ok(())
    }
}
