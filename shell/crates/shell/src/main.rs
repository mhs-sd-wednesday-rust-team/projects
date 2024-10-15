use backend::Backend;
use ir::{CallCommand, Command, PipeCommand};

mod backend;
mod frontend;
mod ir;

fn main() {
    /* Simple example of running
    use std::collections::HashMap;

    let call_command = Command::CallCommand(CallCommand {
        envs: HashMap::new(),
        argv: vec!["echo".to_string(), "Hello, world!".to_string()],
    });


    let mut executor = Backend;
    executor.exec(call_command).expect("Expeceted ok");
    */
}
