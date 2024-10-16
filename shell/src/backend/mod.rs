use std::{
    error::Error,
    process::{Command as ProcessCommand, Output, Stdio},
};

use crate::builtins::exit::ExitCommand;
use crate::ir::CallCommand;
use crate::{
    builtins::{
        cat::CatCommand, echo::EchoCommand, pwd::PwdCommand, wc::WcCommand, BuiltinCommand,
    },
    ir::PipeCommand,
};

pub struct Backend;

#[derive(Debug)]
pub struct ExitStatus {
    code: Option<i32>,
}

impl ExitStatus {
    pub fn new(code: Option<i32>) -> Self {
        Self { code }
    }

    pub fn code(&self) -> Option<i32> {
        self.code
    }
}

impl Default for ExitStatus {
    fn default() -> Self {
        Self { code: None }
    }
}

impl Backend {
    pub fn new() -> Self {
        Self
    }

    /// Executes given ir::PipeCommand
    ///
    /// # Errors
    ///
    /// This function will return an UnimplementedError for two or more commands in
    /// PipeCommand. Moreover, it will return any OS errors encountered during spawn
    /// of subprocess
    pub fn exec(&self, mut pipe: PipeCommand) -> Result<ExitStatus, Box<dyn Error>> {
        if pipe.commands.len() == 0 {
            Ok(ExitStatus::default())
        } else if pipe.commands.len() == 1 {
            let command = pipe.commands.pop().unwrap();
            self.exec_command(command)
        } else {
            Err("pipes are not yet implemented".into())
        }
    }

    /// Executes given ir::Command
    ///
    /// # Errors
    ///
    /// This function will return an UnimplementedError for two or more commands in
    /// PipeCommand. Moreover, it will return any OS errors encountered during spawn
    /// of subprocess
    pub fn exec_command(&self, call_command: CallCommand) -> Result<ExitStatus, Box<dyn Error>> {
        let cmd = call_command.argv[0].clone();
        match cmd.as_str() {
            "cat" => CatCommand::exec(call_command.argv),
            "echo" => EchoCommand::exec(call_command.argv),
            "exit" => ExitCommand::exec(call_command.argv),
            "pwd" => PwdCommand::exec(call_command.argv),
            "wc" => WcCommand::exec(call_command.argv),
            _ => self
                .exec_command_with_io(
                    call_command,
                    Stdio::inherit(),
                    Stdio::inherit(),
                    Stdio::inherit(),
                )
                .map(|output| {
                    output
                        .map(|o| ExitStatus::new(o.status.code()))
                        .unwrap_or_default()
                }),
        }
    }

    fn exec_command_with_io(
        &self,
        call_command: CallCommand,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> Result<Option<Output>, Box<dyn Error>> {
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
        let command = CallCommand {
            envs: HashMap::new(),
            argv: vec!["echo".to_string(), test_str.to_string()],
        };

        let execution_result = backend.exec_command_with_io(
            command,
            Stdio::piped(),
            Stdio::piped(),
            Stdio::piped(),
        )?;
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
    fn test_call_command_passes_custom_environment() -> Result<(), Box<dyn Error>> {
        let backend = Backend;
        let test_key = "some_key";
        let test_value = "some_value";
        let command = CallCommand {
            envs: HashMap::from([(test_key.to_string(), test_value.to_string())]),
            argv: vec!["env".to_string()],
        };

        let execution_result = backend.exec_command_with_io(
            command,
            Stdio::piped(),
            Stdio::piped(),
            Stdio::piped(),
        )?;
        let Some(output) = execution_result else {
            panic!("Expected to spawn some and get its output");
        };

        assert_eq!(Some(0), output.status.code());

        let stdout_content = String::from_utf8_lossy(&output.stdout);
        assert!(stdout_content.contains(&format!("{}={}\n", test_key, test_value)));

        Ok(())
    }

    #[test]
    fn test_failing_command_does_not_fail_shell() -> Result<(), Box<dyn Error>> {
        let backend = Backend;
        let command = CallCommand {
            envs: HashMap::new(),
            argv: vec!["sh".to_string(), "-c".to_string(), r#"exit 5"#.to_string()],
        };

        let execution_result = backend.exec_command_with_io(
            command,
            Stdio::piped(),
            Stdio::piped(),
            Stdio::piped(),
        )?;
        let Some(output) = execution_result else {
            panic!("Expected to spawn some and get its output");
        };

        assert_eq!(Some(5), output.status.code());
        Ok(())
    }
}
