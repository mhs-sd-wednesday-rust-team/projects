use std::error::Error;
use crate::backend::ExitStatus;

pub mod cat;
pub mod echo;
pub mod pwd;
pub mod wc;
pub mod exit;

pub trait BuiltinCommand {
    fn exec(args: Vec<String>) -> Result<ExitStatus, Box<dyn Error>>;
}
