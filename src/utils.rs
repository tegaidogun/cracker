// Utility functions for the Cracker shell

/// Checks if a given command is a built-in command
pub fn is_builtin(command: &str) -> bool {
    match command {
        "cd" | "exit" | "help" | "echo" | "pwd" => true,
        _ => false,
    }
}

/// Splits a command line input into command and arguments
pub fn split_command_line(input: &str) -> (String, Vec<String>) {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let args = parts.map(|s| s.to_string()).collect();
    (command, args)
}
