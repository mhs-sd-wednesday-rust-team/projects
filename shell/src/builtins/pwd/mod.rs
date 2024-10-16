use crate::backend::ExitStatus;
use std::env::current_dir;

use super::BuiltinCommand;

/// Implements the pwd built-in command.
///
/// PwdCommand retrieves and prints the current working directory to the standard output.
pub struct PwdCommand;

impl BuiltinCommand for PwdCommand {
    fn exec(_: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
        let path = current_dir()?;
        println!("{}", path.display());
        Ok(ExitStatus::default())
    }
}
