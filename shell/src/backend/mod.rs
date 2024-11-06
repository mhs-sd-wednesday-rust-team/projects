use std::{
    collections::VecDeque,
    error::Error,
    io::{self, Read, Write},
    process::{Command as ProcessCommand, Stdio},
    thread::{self, JoinHandle},
};

use crate::ir::CallCommand;
use crate::ir::PipeCommand;

pub struct Backend;

/// Represents the exit status of a command execution.
#[derive(Debug, Default)]
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

/// Represents the backend that handles the execution of shell commands.
impl Backend {
    pub fn new() -> Self {
        Self
    }

    /// Executes a sequence of shell commands connected by pipes with specified input/output streams.
    ///
    /// This method sets up and runs a sequence of commands (provided as `PipeCommand`) that form
    /// a pipeline where the output of each command is connected to the input of the next.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Output))` containing the output of the last command, if the execution is successful.
    /// Returns `Err` if an error occurs during the setup or execution of the pipeline, such as if
    /// the command fails to start.
    ///
    /// The function will also return an error if the `PipeCommand` contains fewer than two commands
    /// since a meaningful pipeline requires at least two commands for data flow.
    ///
    /// # Error Handling
    ///
    /// This function captures standard error of the processes by inheriting stderr from the parent.
    /// Errors such as failure in spawning a command or incorrect setup will return a boxed
    /// error encompassing the issue.
    ///
    /// # Panics
    ///
    /// This function may panic if called with invalid `Stdio` objects (e.g., if trying to use the same
    /// `Stdio` handle multiple times or after it has been transformed into a file descriptor).
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
            return Ok(ExitStatus::new(Some(0)));
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
                    command
                        .spawn()?
                        .wait()
                        .map(|status| ExitStatus::new(status.code()))
                        .map_err(|err| format!("{}", err).into())
                })
            }
            crate::ir::Command::Builtin(builtin_command) => thread::spawn(move || {
                let mut stdin = stdin;
                let mut stderr = io::stderr();
                let mut stdout = stdout;
                match builtin_command.exec(call_command.argv, &mut stdin, &mut stderr, &mut stdout)
                {
                    Ok(_) => Ok(ExitStatus::new(Some(0))),
                    Err(err) => {
                        writeln!(stderr, "{}", err)
                            .map_err(|_| "failed to write error to stderr")?;
                        Ok(ExitStatus::new(Some(1)))
                    }
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builtins::{echo::EchoCommand, grep::GrepCommand},
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
        assert!(matches!(status.code(), Some(0)));

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;

        assert_eq!(stdout_output, format!("{}\n", test_str));

        Ok(())
    }

    #[test]
    fn test_call_command_passes_custom_environment() -> Result<(), Box<dyn Error + Sync + Send>> {
        let backend = Backend::new();
        let test_key = "some_key";
        let test_value = "some_value";
        let command = PipeCommand {
            commands: vec![
                CallCommand {
                    envs: HashMap::from([(test_key.to_string(), test_value.to_string())]),
                    command: Command::Call,
                    argv: vec!["env".to_string()],
                },
                CallCommand {
                    argv: vec!["grep".into(), "^some_key=".into()],
                    command: Command::Call,
                    envs: HashMap::new(),
                },
            ],
        };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (mut stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(command, stdin_reader, stdout_writer)?;
        assert!(matches!(status.code(), Some(0)));

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;

        assert_eq!(stdout_output, format!("{}={}\n", test_key, test_value));

        Ok(())
    }

    #[test]
    fn test_failing_command_does_not_fail_shell() -> Result<(), Box<dyn Error + Sync + Send>> {
        let backend = Backend::new();
        let command = PipeCommand {
            commands: vec![CallCommand {
                envs: HashMap::new(),
                command: Command::Call,
                argv: vec!["sh".to_string(), "-c".to_string(), "exit 5".to_string()],
            }],
        };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (_stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(command, stdin_reader, stdout_writer)?;
        assert!(matches!(status.code(), Some(5)));

        Ok(())
    }

    #[test]
    fn test_empty_pipes_do_not_execute() -> Result<(), Box<dyn Error + Sync + Send>> {
        let backend = Backend::new();
        let pipe_command = PipeCommand { commands: vec![] };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (_stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(pipe_command, stdin_reader, stdout_writer)?;
        assert!(matches!(status.code(), Some(0)));

        Ok(())
    }

    #[test]
    fn test_two_command_pipe() -> Result<(), Box<dyn Error + Sync + Send>> {
        let backend = Backend::new();
        let pipe_command = PipeCommand {
            commands: vec![
                CallCommand {
                    argv: vec!["echo".into(), "Hello".into()],
                    command: Command::Builtin(Box::<EchoCommand>::default()),
                    envs: HashMap::new(),
                },
                CallCommand {
                    argv: vec!["grep".into(), "Hello".into()],
                    command: Command::Builtin(Box::<GrepCommand>::default()),
                    envs: HashMap::new(),
                },
            ],
        };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (mut stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(pipe_command, stdin_reader, stdout_writer)?;
        assert!(matches!(status.code(), Some(0)));

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;
        assert_eq!(stdout_output, "Hello\n");

        Ok(())
    }

    #[test]
    fn test_multiple_command_pipe() -> Result<(), Box<dyn Error + Sync + Send>> {
        let backend = Backend::new();
        let pipe_command = PipeCommand {
            commands: vec![
                CallCommand {
                    argv: vec!["echo".into(), "Hello World".into()],
                    command: Command::Builtin(Box::<EchoCommand>::default()),
                    envs: HashMap::new(),
                },
                CallCommand {
                    argv: vec!["tr".into(), "-d".into(), "o".into()],
                    command: Command::Call,
                    envs: HashMap::new(),
                },
                CallCommand {
                    argv: vec!["tr".into(), "-d".into(), "e".into()],
                    command: Command::Call,
                    envs: HashMap::new(),
                },
            ],
        };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (mut stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(pipe_command, stdin_reader, stdout_writer)?;
        assert!(matches!(status.code(), Some(0)));

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;
        assert_eq!(stdout_output, "Hll Wrld\n");

        Ok(())
    }

    #[test]
    fn test_pipe_do_not_stop_on_exit_code() -> Result<(), Box<dyn Error + Send + Sync>> {
        let backend = Backend::new();
        let pipe_command = PipeCommand {
            commands: vec![
                CallCommand {
                    argv: vec!["false".into()],
                    command: Command::Call,
                    envs: HashMap::new(),
                },
                CallCommand {
                    argv: vec!["echo".into(), "Continued".into()],
                    command: Command::Builtin(Box::<EchoCommand>::default()),
                    envs: HashMap::new(),
                },
            ],
        };

        let (stdin_reader, _stdin_writer) = os_pipe::pipe()?;
        let (mut stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = backend.exec(pipe_command, stdin_reader, stdout_writer)?;
        assert!(matches!(status.code(), Some(0)));

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;
        assert_eq!(stdout_output, "Continued\n");

        Ok(())
    }

    #[test]
    fn test_grep_smoke() -> Result<(), Box<dyn Error + Send + Sync>> {
        let backend = Backend::new();
        let pipe_command = PipeCommand {
            commands: vec![CallCommand {
                envs: HashMap::new(),
                command: Command::Builtin(Box::<GrepCommand>::default()),
                argv: vec![
                    "grep".to_string(),
                    "-A".to_string(),
                    "2".to_string(),
                    "Red".to_string(),
                    "-".to_string(),
                ],
            }],
        };

        let (stdin_reader, mut stdin_writer) = os_pipe::pipe()?;
        let (mut stdout_reader, stdout_writer) = os_pipe::pipe()?;

        let status = thread::spawn(move || backend.exec(pipe_command, stdin_reader, stdout_writer));

        write!(
            stdin_writer,
            "A\nRed1\nA\nG\nC\nRed2\nE\nF\nRedRed\nRed\nBlueRed\nReed"
        )
        .unwrap();
        drop(stdin_writer);

        assert!(matches!(status.join().unwrap().unwrap().code(), Some(0)));

        let mut stdout_output = String::new();
        stdout_reader.read_to_string(&mut stdout_output)?;

        assert_eq!(
            stdout_output,
            format!(
                "{}",
                "Red1\nA\nG\n--\nRed2\nE\nF\nRedRed\nRed\nBlueRed\nReed\n"
            )
        );

        Ok(())
    }
}
