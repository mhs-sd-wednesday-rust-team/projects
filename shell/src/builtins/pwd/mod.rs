use std::env::current_dir;
use crate::backend::ExitStatus;

use super::BuiltinCommand;

pub struct PwdCommand;

impl BuiltinCommand for PwdCommand {
    fn exec(_: Vec<String>) -> Result<ExitStatus, Box<dyn std::error::Error>> {
        let path = current_dir()?;
        println!("{}", path.display());
        Ok(ExitStatus::default())
    }
}
