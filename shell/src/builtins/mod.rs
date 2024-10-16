use crate::backend::ExitStatus;
use std::error::Error;

pub mod cat;
pub mod echo;
pub mod exit;
pub mod pwd;
pub mod wc;

pub trait BuiltinCommand {
    fn exec(args: Vec<String>) -> Result<ExitStatus, Box<dyn Error>>;
}
