use crate::shell::utils::path_utils::resolve_path;
use std::env;

pub fn cd(args: &[&str]) -> Result<(), String> {
    let mut follow_symlinks = true;
    let mut check_symlinks = false;
    let mut show_attributes = false;

    let mut directory: Option<&str> = None;

    for arg in args {
        match *arg {
            "-L" => follow_symlinks = true,
            "-P" => follow_symlinks = false,
            "-e" => check_symlinks = true,
            "-@" => show_attributes = true,
            _ => {
                if directory.is_none() {
                    directory = Some(arg);
                }
            }
        }
    }

    let directory = directory.unwrap_or("~");

    if directory == "-" {
        if let Ok(oldpwd) = env::var("OLDPWD") {
            println!("{}", oldpwd);
            change_directory(&oldpwd, follow_symlinks, check_symlinks, show_attributes)
        } else {
            Err("cd: OLDPWD not set".into())
        }
    } else {
        change_directory(directory, follow_symlinks, check_symlinks, show_attributes)
    }
}

fn change_directory(
    dir: &str,
    follow_symlinks: bool,
    check_symlinks: bool,
    show_attributes: bool,
) -> Result<(), String> {
    let path = resolve_path(dir);

    let final_path = if follow_symlinks {
        path.canonicalize().unwrap_or(path.clone())
    } else {
        path
    };

    if show_attributes {
        // This is a placeholder for handling extended attributes.
        // You would use platform-specific APIs to fetch and display extended attributes here.
        println!("Attributes for {}: ...", final_path.display());
    }

    if check_symlinks && !final_path.exists() {
        return Err("cd: symlink resolution failed".into());
    }

    let current_dir = env::current_dir().map_err(|e| format!("cd: {}", e))?;
    env::set_var("OLDPWD", current_dir);

    env::set_current_dir(&final_path).map_err(|e| format!("cd: {}", e))?;
    let new_dir = env::current_dir().map_err(|e| format!("cd: {}", e))?;
    env::set_var("PWD", &new_dir);

    println!("{}", new_dir.display());

    Ok(())
}
