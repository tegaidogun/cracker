use std::env;
use std::fmt;

/// Custom error type for shell-related errors.
#[derive(Debug)]
pub enum ShellError {
    CommandNotFound(String),
    PathNotFound(String),
    GeneralError(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::CommandNotFound(cmd) => write!(f, "Command not found: {}", cmd),
            ShellError::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ShellError::GeneralError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ShellError {}

/// Function to handle errors and print them to the user.
pub fn handle_error(error: ShellError) {
    eprintln!("{}", error);
}

/// Adds custom user session paths like ~/.local/bin to the current PATH.
pub fn add_user_session_paths() -> Result<(), ShellError> {
    let home_dir = dirs::home_dir().ok_or(ShellError::PathNotFound("Home directory".into()))?;
    
    let custom_paths = vec![
        home_dir.join(".local/bin"),
        home_dir.join("bin"),
        home_dir.join(".local/share/bin"),
    ];

    let current_path = env::var("PATH").unwrap_or_else(|_| "".to_string());
    let mut new_path = env::split_paths(&current_path).collect::<Vec<_>>();

    for path in custom_paths {
        if !new_path.contains(&path) {
            new_path.push(path);
        }
    }

    let new_path_string = env::join_paths(new_path)
        .map_err(|_| ShellError::GeneralError("Failed to update PATH".into()))?;

    env::set_var("PATH", new_path_string);
    Ok(())
}
