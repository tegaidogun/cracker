pub mod commands;
pub mod utils;
pub mod error;

use std::env;
use std::io::{self, Write};
use std::process::Command;
use crate::shell::utils::path_utils::to_tilde_path;
use crate::shell::error::{check_command_executable, handle_error, ShellError};

fn execute_external(command: &str, args: &[&str]) -> Result<(), ShellError> {
    match check_command_executable(command) {
        Ok(command_path) => {
            let result = Command::new(command_path).args(args).status();

            match result {
                Ok(status) => {
                    if !status.success() {
                        return Err(ShellError::GeneralError(format!("Command {} exited with status code {}", command, status)));
                    }
                }
                Err(e) => return Err(ShellError::GeneralError(format!("Failed to execute {}: {}", command, e))),
            }

            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub fn run() {
    // Adding user session paths to PATH
    if let Err(err) = crate::shell::error::add_user_session_paths() {
        crate::shell::error::handle_error(err);
    }

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
                }
            }
            "cp" => {
                if let Err(e) = crate::shell::commands::cp::cp(&args) {
                    eprintln!("{}", e);
                }
            }
            "clear" => crate::shell::commands::clear::clear(),
            "echo" => crate::shell::commands::echo::echo(&args),
            "exit" => break,
            _ => {
                if let Err(e) = execute_external(command, &args) {
                    handle_error(e);
                }
            }
        }
    }
}
