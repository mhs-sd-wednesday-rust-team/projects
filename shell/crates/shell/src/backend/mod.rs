use std::{
    error::Error,
    process::{Command as ProcessCommand, Output, Stdio},
};

use crate::ir::Command;

pub struct Backend;

impl Backend {
    pub fn exec(
        &self,
        command: Command,
        // r: R, w: W,
    ) -> Result<(), Box<dyn Error>>
// where
    //     R: Read,
    //     W: Write,
    {
        self.exec_with_io(
            command,
            Stdio::inherit(),
            Stdio::inherit(),
            Stdio::inherit(),
        )
        .map(|_| ())
    }

    fn exec_with_io(
        &self,
        command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> Result<Option<Output>, Box<dyn Error>> {
        match command {
            Command::PipeCommand(_pipe_command) => unimplemented!(),
            Command::CallCommand(call_command) => {
                let mut process_command = ProcessCommand::new(&call_command.argv[0]);
                process_command
                    .args(&call_command.argv[1..])
                    .stdin(stdin)
                    .stdout(stdout)
                    .stderr(stderr)
                    .envs(call_command.envs);

                let output = process_command.spawn()?.wait_with_output()?;
                return Ok(Some(output));
            }
            Command::ExitCommand => {
                return Ok(None);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::CallCommand;
    use std::collections::HashMap;

    #[test]
    fn test_call_command_stdout() -> Result<(), Box<dyn Error>> {
        let backend = Backend;
        let test_str = "Hello, world!";
        let command = Command::CallCommand(CallCommand {
            envs: HashMap::new(),
            argv: vec!["echo".to_string(), test_str.to_string()],
        });

        let execution_result =
            backend.exec_with_io(command, Stdio::piped(), Stdio::piped(), Stdio::piped())?;
        let Some(output) = execution_result else {
            panic!("Expected to spawn some and get its output");
        };

        assert_eq!(Some(0), output.status.code());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!("{}\n", test_str)
        );

        Ok(())
    }

    #[test]
    fn test_failing_command_does_not_fail_shell() -> Result<(), Box<dyn Error>> {
        let backend = Backend;
        let command = Command::CallCommand(CallCommand {
            envs: HashMap::new(),
            argv: vec!["sh".to_string(), "-c".to_string(), r#"exit 5"#.to_string()],
        });

        let execution_result =
            backend.exec_with_io(command, Stdio::piped(), Stdio::piped(), Stdio::piped())?;
        let Some(output) = execution_result else {
            panic!("Expected to spawn some and get its output");
        };

        assert_eq!(Some(5), output.status.code());
        Ok(())
    }

    #[test]
    fn test_exit_just_returns() -> Result<(), Box<dyn Error>> {
        let backend = Backend;
        let command = Command::ExitCommand;

        let execution_result =
            backend.exec_with_io(command, Stdio::piped(), Stdio::piped(), Stdio::piped())?;
        assert_eq!(None, execution_result);
        Ok(())
    }
}
