use std::env::current_dir;

use super::BuiltinCommand;

pub struct PwdCommand;

impl BuiltinCommand for PwdCommand {
    fn exec(_: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let path = current_dir()?;
        println!("{}", path.display());
        Ok(())
    }
}
