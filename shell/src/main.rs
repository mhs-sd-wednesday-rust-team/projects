use std::error::Error;

use os_pipe::{dup_stdin, dup_stdout};

mod backend;
mod builtins;
mod frontend;
mod ir;

fn main() -> Result<(), Box<dyn Error>> {
    let mut frontend = frontend::Frontend::new();
    let backend = backend::Backend::new();

    for line in std::io::stdin().lines() {
        let line = line?;

        let command = match frontend.parse(&line) {
            Ok(command) => command,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        match backend.exec(command, dup_stdin()?, dup_stdout()?) {
            Ok(exit_status) => match exit_status.code() {
                Some(code) if code != 0 => {
                    eprintln!("exited with code {}", code);
                }
                _ => {}
            },
            Err(err) => {
                eprintln!("shell error: {}", err);
                continue;
            }
        };
    }

    Ok(())
}
