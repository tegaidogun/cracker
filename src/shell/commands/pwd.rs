use crate::shell::utils::path_utils::resolve_path;
use std::env;

pub fn pwd(args: &[&str]) -> Result<(), String> {
    let mut resolve_symlinks = false;

    for arg in args {
        match *arg {
            "-P" => resolve_symlinks = true,
            "-L" => resolve_symlinks = false,
            _ => return Err(format!("pwd: invalid option -- '{}'", arg)),
        }
    }

    let current_dir = env::current_dir().map_err(|e| format!("pwd: {}", e))?;

    let display_dir = if resolve_symlinks {
        resolve_path(&current_dir.to_string_lossy()).canonicalize().map_err(|e| format!("pwd: {}", e))?
    } else {
        current_dir
    };

    println!("{}", display_dir.display());
    Ok(())
}
