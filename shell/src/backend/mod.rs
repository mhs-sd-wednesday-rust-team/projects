use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{self, Read, Write},
    process::{Command as ProcessCommand, Stdio},
    thread::{self, JoinHandle},
};

use crate::ir::CallCommand;
use crate::ir::PipeCommand;

pub struct Backend;

/// Represents the exit status of a command execution.
pub type ExitStatus = Result<(), i32>;

/// Represents the backend that handles the execution of shell commands.
impl Backend {
    pub fn new() -> Self {
        Self
    }

    /// Executes given ir::PipeCommand
    ///
    /// # Errors
    ///
    /// This function will return an Err for two or more commands in
    /// PipeCommand. Moreover, it will return any OS errors encountered during spawn
    /// of subprocess
    pub fn exec<Stdin, Stdout>(
        &self,
        mut pipe: PipeCommand,
        stdin: Stdin,
        stdout: Stdout,
    ) -> Result<ExitStatus, Box<dyn Error + Sync + Send>>
    where
        Stdin: Into<Stdio> + Read + Send + 'static,
        Stdout: Into<Stdio> + Write + Send + 'static,
    {
        if pipe.commands.is_empty() {
            return Ok(ExitStatus::Ok(()));
        } else if pipe.commands.len() == 1 {
            let command = pipe.commands.pop().unwrap();
            let join_res = self.spawn_command(command, stdin, stdout).join();
            return match join_res {
                Ok(res) => res,
                Err(err) => Err(format!("{:?}", err).into()),
            };
        }

        let mut commands = VecDeque::new();
        let mut pipe_commands = pipe.commands.drain(..).collect::<VecDeque<_>>();

        let (mut reader, writer) = os_pipe::pipe()?;
        commands.push_back(self.spawn_command(pipe_commands.pop_front().unwrap(), stdin, writer));

        while pipe_commands.len() != 1 {
            let next_cmd = pipe_commands.pop_front().unwrap();
            let (next_reader, next_writer) = os_pipe::pipe()?;
            commands.push_back(self.spawn_command(next_cmd, reader, next_writer));
            reader = next_reader;
        }

        commands.push_back(self.spawn_command(pipe_commands.pop_front().unwrap(), reader, stdout));

        while commands.len() != 1 {
            let command = commands.pop_front().unwrap();
            let _ = command.join().map_err(|err| format!("{:?}", err))?;
        }
        commands
            .pop_front()
            .unwrap()
            .join()
            .map_err(|err| format!("{:?}", err))?
    }

    /// Executes given ir::Command
    ///
    /// # Errors
    ///
    /// This function will return an UnimplementedError for two or more commands in
    /// PipeCommand. Moreover, it will return any OS errors encountered during spawn
    /// of subprocess
    pub fn spawn_command<Stdin, Stdout>(
        &self,
        call_command: CallCommand,
        stdin: Stdin,
        stdout: Stdout,
    ) -> JoinHandle<Result<ExitStatus, Box<dyn Error + Send + Sync>>>
    where
        Stdin: Into<Stdio> + Read + Send + 'static,
        Stdout: Into<Stdio> + Write + Send + 'static,
    {
        match call_command.command {
            crate::ir::Command::Call => {
                let mut command = ProcessCommand::new(&call_command.argv[0]);

                command
                    .args(&call_command.argv[1..])
                    .stdin(stdin)
                    .stdout(stdout)
                    .stderr(Stdio::inherit())
                    .envs(call_command.envs);

                thread::spawn(move || {
                    let status = command.spawn()?.wait()?;
                    match status.code() {
                        Some(0) => Ok(ExitStatus::Ok(())),
                        Some(code) => Ok(ExitStatus::Err(code)),
                        None => Err("no exit code".into()),
                    }
                })
            }
            crate::ir::Command::Builtin(builtin_command) => thread::spawn(move || {
                let mut stdin = stdin;
                let mut stderr = io::stderr();
                let mut stdout = stdout;
                Ok(builtin_command.exec(call_command.argv, &mut stdin, &mut stderr, &mut stdout))
            }),
        }
    }

    // Executes a sequence of shell commands connected by pipes with specified input/output streams.
    //
    // This method sets up and runs a sequence of commands (provided as `PipeCommand`) that form
    // a pipeline where the output of each command is connected to the input of the next.
    //
    //
    // # Returns
    //
    // Returns `Ok(Some(Output))` containing the output of the last command, if the execution is successful.
    // Returns `Err` if an error occurs during the setup or execution of the pipeline, such as if
    // the command fails to start.
    //
    // The function will also return an error if the `PipeCommand` contains fewer than two commands
    // since a meaningful pipeline requires at least two commands for data flow.
    //
    // # Error Handling
    //
    // This function captures standard error of the processes by inheriting stderr from the parent.
    // Errors such as failure in spawning a command or incorrect setup will return a boxed
    // error encompassing the issue.
    //
    // # Panics
    //
    // This function may panic if called with invalid `Stdio` objects (e.g., if trying to use the same
    // `Stdio` handle multiple times or after it has been transformed into a file descriptor).
    // fn exec_pipe_command_with_io(
    //     &self,
    //     pipe_command: PipeCommand,
    //     stdin: Stdio,
    //     stdout: Stdio,
    // ) -> Result<Option<Output>, Box<dyn Error>> {
    //     let commands = pipe_command.commands;
    //     if commands.len() < 2 {
    //         return Err("Pipe must contain at least two commands".into());
    //     }

    //     // FIXME: нужно научиться запускать наши built-in как процессы, тогда тут можно будет порефачить и заюзать общий метод для запуска CallCommand
    //     let first_comamnd = ProcessCommand::new(&commands[0].argv[0])
    //         .args(&commands[0].argv[1..])
    //         .stdin(stdin)
    //         .stdout(Stdio::piped())
    //         .stderr(Stdio::inherit())
    //         .envs(commands[0].envs.clone())
    //         .spawn()?;
    //     let mut prev_command = first_comamnd;
    //     for next_cmd in commands[1..commands.len() - 1].iter() {
    //         prev_command = ProcessCommand::new(&next_cmd.argv[0])
    //             .args(&next_cmd.argv[1..])
    //             .stdin(Stdio::from(prev_command.stdout.unwrap()))
    //             .stdout(Stdio::piped())
    //             .stderr(Stdio::inherit())
    //             .envs(next_cmd.envs.clone())
    //             .spawn()?;
    //     }

    //     let last_cmd = commands.last().unwrap();
    //     let final_command = ProcessCommand::new(&last_cmd.argv[0])
    //         .args(&last_cmd.argv[1..])
    //         .stdin(Stdio::from(prev_command.stdout.unwrap()))
    //         .stdout(stdout)
    //         .stderr(Stdio::inherit())
    //         .envs(last_cmd.envs.clone())
    //         .spawn()?;

    //     Ok(Some(final_command.wait_with_output()?))
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builtins::echo::EchoCommand,
        ir::{CallCommand, Command},
    };
    use std::collections::HashMap;

    #[test]
    fn test_call_command_stdout() -> Result<(), Box<dyn Error + Send + Sync>> {
        let backend = Backend::new();
        let test_str = "Hello, world!";
        let command = PipeCommand {
            commands: vec![CallCommand {
                envs: HashMap::new(),
                command: Command::Builtin(Box::<EchoCommand>::default()),
                argv: vec!["echo".to_string(), test_str.to_string()],
            }],
        };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (mut stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(command, stdin_reader, stdout_writer)?;
        assert!(status.is_ok());

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;

        assert_eq!(stdout_output, format!("{}\n", test_str));

        Ok(())
    }

    // #[test]
    // fn test_call_command_passes_custom_environment() -> Result<(), Box<dyn Error>> {
    //     let backend = Backend;
    //     let test_key = "some_key";
    //     let test_value = "some_value";
    //     let command = CallCommand {
    //         envs: HashMap::from([(test_key.to_string(), test_value.to_string())]),
    //         argv: vec!["env".to_string()],
    //     };

    //     let execution_result = backend.exec_command_with_io(
    //         command,
    //         Stdio::piped(),
    //         Stdio::piped(),
    //         Stdio::piped(),
    //     )?;
    //     let Some(output) = execution_result else {
    //         panic!("Expected to spawn some and get its output");
    //     };

    //     assert_eq!(Some(0), output.status.code());

    //     let stdout_content = String::from_utf8_lossy(&output.stdout);
    //     assert!(stdout_content.contains(&format!("{}={}\n", test_key, test_value)));

    //     Ok(())
    // }

    // #[test]
    // fn test_failing_command_does_not_fail_shell() -> Result<(), Box<dyn Error>> {
    //     let backend = Backend;
    //     let command = CallCommand {
    //         envs: HashMap::new(),
    //         argv: vec!["sh".to_string(), "-c".to_string(), "exit 5".to_string()],
    //     };

    //     let execution_result = backend.exec_command_with_io(
    //         command,
    //         Stdio::piped(),
    //         Stdio::piped(),
    //         Stdio::piped(),
    //     )?;
    //     let Some(output) = execution_result else {
    //         panic!("Expected to spawn some and get its output");
    //     };

    //     assert_eq!(Some(5), output.status.code());
    //     Ok(())
    // }

    // #[test]
    // fn test_empty_pipes_do_not_execute() -> Result<(), Box<dyn Error>> {
    //     let backend = Backend::new();
    //     let pipe_command = PipeCommand { commands: vec![] };
    //     let result = backend.exec_pipe_command_with_io(pipe_command, Stdio::null(), Stdio::null());
    //     assert!(result.is_err(), "Expected an error for empty pipe");
    //     Ok(())
    // }

    // #[test]
    // fn test_two_command_pipe() -> Result<(), Box<dyn Error>> {
    //     let backend = Backend::new();
    //     let pipe_command = PipeCommand {
    //         commands: vec![
    //             CallCommand {
    //                 argv: vec!["echo".into(), "Hello".into()],
    //                 envs: HashMap::new(),
    //             },
    //             CallCommand {
    //                 argv: vec!["grep".into(), "Hello".into()],
    //                 envs: HashMap::new(),
    //             },
    //         ],
    //     };

    //     let output =
    //         backend.exec_pipe_command_with_io(pipe_command, Stdio::null(), Stdio::piped())?;
    //     assert!(output.is_some());
    //     let output = output.unwrap();
    //     assert!(output.status.success());
    //     assert!(String::from_utf8_lossy(&output.stdout).contains("Hello"));
    //     Ok(())
    // }

    #[test]
    fn test_multiple_command_pipe() -> Result<(), Box<dyn Error>> {
        let backend = Backend::new();
        let pipe_command = PipeCommand {
            commands: vec![
                CallCommand {
                    argv: vec!["echo".into(), "Hello World".into()],
                    envs: HashMap::new(),
                },
                CallCommand {
                    argv: vec!["tr".into(), "-d".into(), "o".into()],
                    envs: HashMap::new(),
                },
                CallCommand {
                    argv: vec!["tr".into(), "-d".into(), "e".into()],
                    envs: HashMap::new(),
                },
            ],
        };

        let output =
            backend.exec_pipe_command_with_io(pipe_command, Stdio::null(), Stdio::piped())?;
        assert!(output.is_some());
        let output = output.unwrap();
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hll Wrld"));
        Ok(())
    }

    // #[test]
    // fn test_pipe_do_not_stop_on_exit_code() -> Result<(), Box<dyn Error>> {
    //     let backend = Backend::new();
    //     let pipe_command = PipeCommand {
    //         commands: vec![
    //             CallCommand {
    //                 argv: vec!["false".into()],
    //                 envs: HashMap::new(),
    //             },
    //             CallCommand {
    //                 argv: vec!["echo".into(), "Continued".into()],
    //                 envs: HashMap::new(),
    //             },
    //         ],
    //     };

    //     let output =
    //         backend.exec_pipe_command_with_io(pipe_command, Stdio::null(), Stdio::piped())?;

    //     assert!(output.is_some());
    //     let output = output.unwrap();
    //     assert!(String::from_utf8_lossy(&output.stdout).contains("Continued"));
    //     Ok(())
    // }
}
