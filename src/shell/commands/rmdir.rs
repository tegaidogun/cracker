use std::fs;
use std::path::Path;
use crate::shell::utils::path_utils::resolve_path;
use std::io::ErrorKind;

pub fn rmdir(args: &[&str]) -> Result<(), String> {
    let mut ignore_fail_on_non_empty = false;
    let mut remove_parents = false;
    let mut verbose = false;

    let mut paths = Vec::new();

    for arg in args {
        match *arg {
            "--ignore-fail-on-non-empty" => ignore_fail_on_non_empty = true,
            "-p" | "--parents" => remove_parents = true,
            "--verbose" => verbose = true,
            "--" => break, // Stops option parsing
            _ => paths.push(resolve_path(arg)),
        }
    }

    if paths.is_empty() {
        return Err("rmdir: missing operand".into());
    }

    for path in paths {
        if !path.exists() {
            return Err(format!("rmdir: failed to remove '{}': No such file or directory", path.display()));
        }

        if remove_parents {
            let mut current_path = path.as_path();
            while current_path != Path::new("") {
                if let Err(e) = fs::remove_dir(current_path) {
                    if !ignore_fail_on_non_empty || e.kind() != ErrorKind::Other {
                        return Err(format!("rmdir: failed to remove '{}': {}", current_path.display(), e));
                    }
                } else if verbose {
                    println!("removed directory '{}'", current_path.display());
                }

                current_path = current_path.parent().unwrap_or_else(|| Path::new(""));
            }
        } else {
            if let Err(e) = fs::remove_dir(&path) {
                if !ignore_fail_on_non_empty || e.kind() != ErrorKind::Other {
                    return Err(format!("rmdir: failed to remove '{}': {}", path.display(), e));
                }
            } else if verbose {
                println!("removed directory '{}'", path.display());
            }
        }
    }

    Ok(())
}
