pub mod utils;

use std::env;
use std::io::{self, Write};
use std::process::Command;
use crate::shell::utils::path_utils::{resolve_path, to_tilde_path};

pub fn run() {
    loop {
        // Get the current working directory
        let current_dir = env::current_dir().unwrap_or_else(|_| "unknown".into());
        let display_dir = to_tilde_path(&current_dir);
        let prompt = format!("crash | {}> ", display_dir);

        // Display the prompt
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read the input
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read line: {}", error);
            continue;
        }

        // Trim the input and check for empty commands
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Built-in command: exit
        if input == "exit" {
            break;
        }

        // Attempt to resolve the command path
        let resolved_command = resolve_path(input);
        let command_str = resolved_command.to_str().unwrap_or(input);

        // Attempt to execute the command as an external program
        execute_external(command_str);
    }
}

fn execute_external(command: &str) {
    let mut parts = command.split_whitespace();
    if let Some(executable) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let result = Command::new(executable).args(&args).status();

        match result {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command {} exited with status code {}", executable, status);
                }
            }
            Err(e) => eprintln!("Failed to execute {}: {}", executable, e),
        }
    }
}
