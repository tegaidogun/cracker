use std::io::{self, Write};
use crate::builtins;
use crate::commands;
use crate::utils;

pub fn start() {
    loop {
        // Display the prompt
        print!("cracker> ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Skip empty input
        if input.is_empty() {
            continue;
        }

        // Split the input into command and arguments
        let (command, args) = utils::split_command_line(input);

        // Check for built-in commands
        if utils::is_builtin(&command) {
            if builtins::handle_builtin(&command, &args) {
                continue;
            }
        } else {
            // Handle external commands
            if commands::execute(&command, &args) {
                continue;
            }
        }

        // If the command is neither built-in nor an external command
        eprintln!("cracker: command not found: {}", input);
    }
}
