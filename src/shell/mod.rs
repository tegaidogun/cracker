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
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::io;

use std::io::{Write};

pub fn start_shell(show_pwd: bool) {
    loop {
        // Get the current working directory and home directory
        let current_dir = env::current_dir().unwrap_or_else(|_| "unknown".into());
        let home_dir = get_home_dir().unwrap_or_else(|| PathBuf::from("unknown"));

        // Determine the display path
        let display_path = if current_dir.starts_with(&home_dir) {
            format!("~{}", current_dir.strip_prefix(&home_dir).unwrap().display())
        } else {
            current_dir.display().to_string()
        };

        // Prepare the prompt with or without the current directory
        let prompt = if show_pwd {
            format!("🧇 | {}> ", display_path)
        } else {
            "🧇 | > ".to_string()
        };

        // Display the shell prompt
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Expand environment variables and home directory
        let input = expand_env_vars(input.trim());
        let input = expand_home_dir(&input);

        // Trim and split the input into command and arguments
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        // Handle built-in commands or execute external command
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
                if let Some(status) = execute_external_command(command, &args) {
                    if !status.success() {
                        eprintln!("Command `{}` exited with status: {}", command, status);
                    }
                } else {
                    eprintln!("cracker: command not found: {}", command);
                }
            }
        }
    }
}

fn execute_external_command(command: &str, args: &[&str]) -> Option<ExitStatus> {
    if let Some(path) = find_command_in_path(command) {
        match Command::new(path).args(args).status() {
            Ok(status) => Some(status),
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
                None
            }
        }
    } else {
        None
    }
}

fn find_command_in_path(command: &str) -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    let current_path = current_dir.join(command);
    if is_executable(&current_path) {
        return Some(current_path);
    }

    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(command);
            if is_executable(&full_path) {
                return Some(full_path);
            }
        }
    }
    None
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.is_file() && path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
    }

    #[cfg(windows)]
    {
        // On Windows, we check if the file is executable by attempting to start it.
        path.is_file() && path.extension().and_then(|ext| ext.to_str()).map_or(false, |ext| {
            ["exe", "bat", "cmd", "com"].contains(&ext.to_lowercase().as_str())
        })
    }
}

fn get_home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        env::var("HOME").ok().map(PathBuf::from)
    }

    #[cfg(windows)]
    {
        env::var("USERPROFILE").ok().map(PathBuf::from)
    }
}

fn expand_env_vars(input: &str) -> String {
    let mut result = String::new();
    let mut var_name = String::new();
    let mut in_var = false;

    for ch in input.chars() {
        if in_var {
            if ch.is_alphanumeric() || ch == '_' {
                var_name.push(ch);
            } else {
                if let Ok(var_value) = env::var(&var_name) {
                    result.push_str(&var_value);
                } else {
                    result.push('$');
                    result.push_str(&var_name);
                }
                var_name.clear();
                in_var = false;
                result.push(ch);
            }
        } else if ch == '$' {
            in_var = true;
        } else {
            result.push(ch);
        }
    }

    if in_var {
        if let Ok(var_value) = env::var(&var_name) {
            result.push_str(&var_value);
        } else {
            result.push('$');
            result.push_str(&var_name);
        }
    }

    result
}

fn expand_home_dir(input: &str) -> String {
    if input.starts_with('~') {
        if let Some(home_dir) = get_home_dir() {
            return input.replacen("~", &home_dir.display().to_string(), 1);
        }
    }
    input.to_string()
}
