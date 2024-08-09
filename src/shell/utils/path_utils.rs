use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolves a given path by expanding environment variables and handling special cases like `~` for home directory.
pub fn resolve_path(path: &str) -> PathBuf {
    // Handle environment variables like $VITASDK
    let expanded_path = shellexpand::env(path).unwrap_or_else(|_| path.into());

    // Convert to a PathBuf
    let mut path_buf = PathBuf::from(expanded_path.to_string());

    // Handle special cases like ~ for home directory
    if path_buf.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            path_buf = home_dir.join(path_buf.strip_prefix("~").unwrap());
        }
    }

    path_buf
}

/// Checks if a command is an executable in the user's PATH.
pub fn is_executable_in_path(command: &str) -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() && is_executable(&full_path) {
                return true;
            }
        }
    }
    false
}

/// Helper function to determine if a path is executable (Unix-specific).
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).map(|meta| meta.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

/// Converts a full path to a path relative to the user's home directory using `~`.
pub fn to_tilde_path(path: &Path) -> String {
    if let Some(home_dir) = dirs::home_dir() {
        if path.starts_with(&home_dir) {
            let relative_path = path.strip_prefix(&home_dir).unwrap();
            return format!("~/{}", relative_path.display());
        }
    }
    path.display().to_string()
}

/// Checks if a given path exists and is a directory.
pub fn path_exists_and_is_dir(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Copies a file or directory from source to destination.
pub fn copy_path(source: &Path, destination: &Path) -> Result<(), String> {
    if source.is_file() {
        fs::copy(source, destination).map_err(|e| e.to_string())?;
    } else if source.is_dir() {
        fs::create_dir_all(destination).map_err(|e| e.to_string())?;
        for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let dest_path = destination.join(entry.file_name());
            copy_path(&entry.path(), &dest_path)?;
        }
    }
    Ok(())
}
