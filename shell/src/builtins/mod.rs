use std::error::Error;

pub mod cat;
pub mod echo;
pub mod pwd;
pub mod wc;

pub trait BuiltinCommand {
    fn exec(args: Vec<String>) -> Result<(), Box<dyn Error>>;
}
