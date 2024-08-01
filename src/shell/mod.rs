pub mod commands;
pub mod utils;

use commands::cat;
use commands::cd;
use commands::cp;
use commands::ls;
use commands::mkdir;
use commands::mv;
use commands::pwd;
use commands::rm;
use commands::rmdir;
use commands::touch;
use commands::help;
use commands::clear;
use commands::echo;

use std::env;
use std::io::{self, Write};

pub fn start_shell() {
    loop {
        // Get the current working directory
        let current_dir = env::current_dir().unwrap_or_else(|_| "unknown".into());
        let prompt = format!("cracker | {}> ", current_dir.display());

        // Display the shell prompt
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim and split the input into command and arguments
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        // Handle built-in commands
        match command {
            "cd" => cd::cd(args),
            "ls" => ls::ls(args),
            "cp" => cp::cp(args),
            "mkdir" => mkdir::mkdir(args),
            "mv" => mv::mv(args),
            "pwd" => pwd::pwd(),
            "rm" => rm::rm(args),
            "cat" => cat::cat(args),
            "rmdir" => rmdir::rmdir(args),
            "touch" => touch::touch(args),
            "help" => help::help(args),
            "echo" => echo::echo(args),
            "clear" => clear::clear(),
            "exit" => break,
            _ => {
                eprintln!("cracker: command not found: {}", command);
            }
        }
    }
}
