pub mod commands;
pub mod error;
pub mod utils;

use crate::shell::error::{handle_error, ShellError};
use crate::shell::utils::path_utils::to_tilde_path;
use std::env;
use std::io::{self, Write};
use std::process::Command;

fn execute_external(command: &str, args: &[&str]) -> Result<(), ShellError> {
    if crate::shell::utils::path_utils::is_executable_in_path(command) {
        let result = Command::new(command).args(args).status();

        match result {
            Ok(status) => {
                if !status.success() {
                    return Err(ShellError::GeneralError(format!(
                        "Command {} exited with status code {}",
                        command, status
                    )));
                }
            }
            Err(e) => {
                return Err(ShellError::GeneralError(format!(
                    "Failed to execute {}: {}",
                    command, e
                )))
            }
        }

        Ok(())
    } else {
        Err(ShellError::CommandNotFound(command.to_string()))
    }
}

pub fn run() {
    // Adding user session paths to PATH
    if let Err(err) = crate::shell::error::add_user_session_paths() {
        crate::shell::error::handle_error(err);
    }

    let mut last_status = 0;

    loop {
        let current_dir = env::current_dir().unwrap_or_else(|_| "unknown".into());
        let display_dir = to_tilde_path(&current_dir);
        let prompt = format!("crash | {}> ", display_dir);

        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read line: {}", error);
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => {
                if let Err(e) = crate::shell::commands::cd::cd(&args) {
                    eprintln!("{}", e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
            "cp" => {
                if let Err(e) = crate::shell::commands::cp::cp(&args) {
                    eprintln!("{}", e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
            "clear" => {
                crate::shell::commands::clear::clear();
                last_status = 0;
            }
            "echo" => {
                if let Err(e) = crate::shell::commands::echo::echo(&args) {
                    eprintln!("{}", e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
            "exit" => crate::shell::commands::exit::exit_shell(&args, last_status),
            "ls" => {
                if let Err(e) = crate::shell::commands::ls::ls(&args) {
                    eprintln!("{}", e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
            "mkdir" => {
                if let Err(e) = crate::shell::commands::mkdir::mkdir(&args) {
                    eprintln!("{}", e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
            "mv" => {
                if let Err(e) = crate::shell::commands::mv::mv(&args) {
                    eprintln!("{}", e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
            _ => {
                if let Err(e) = execute_external(command, &args) {
                    handle_error(e);
                    last_status = 1;
                } else {
                    last_status = 0;
                }
            }
        }
    }
}
