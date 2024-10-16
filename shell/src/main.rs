use std::error::Error;
use std::process::exit;

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

        match backend.exec(command) {
            Ok(exit_status) => match exit_status.code() {
                Some(code) if code != 0 => {
                    eprintln!("exited with code {}", code);
                    exit(code)
                }
                _ => {}
            },
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
    }

    Ok(())
}
