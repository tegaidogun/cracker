use std::fs;
use std::os::unix::fs::PermissionsExt;
use crate::shell::utils::path_utils::resolve_path;

pub fn mkdir(args: &[&str]) -> Result<(), String> {
    let mut mode = None;
    let mut parents = false;
    let mut verbose = false;
    let mut directories = Vec::new();

    for arg in args {
        match *arg {
            "-m" => {
                if let Some(&next_arg) = args.iter().nth(args.iter().position(|&a| a == "-m").unwrap() + 1) {
                    mode = Some(parse_mode(next_arg)?);
                } else {
                    return Err("mkdir: option requires an argument -- 'm'".into());
                }
            }
            "-p" | "--parents" => parents = true,
            "-v" | "--verbose" => verbose = true,
            _ => directories.push(resolve_path(arg)),
        }
    }

    if directories.is_empty() {
        return Err("mkdir: missing operand".into());
    }

    for dir in directories {
        if parents {
            fs::create_dir_all(&dir).map_err(|e| format!("mkdir: {}", e))?;
        } else {
            fs::create_dir(&dir).map_err(|e| format!("mkdir: {}", e))?;
        }

        if let Some(mode) = mode {
            fs::set_permissions(&dir, fs::Permissions::from_mode(mode))
                .map_err(|e| format!("mkdir: cannot set permissions on {}: {}", dir.display(), e))?;
        }

        if verbose {
            println!("mkdir: created directory '{}'", dir.display());
        }
    }

    Ok(())
}

fn parse_mode(mode_str: &str) -> Result<u32, String> {
    u32::from_str_radix(mode_str, 8).map_err(|_| format!("mkdir: invalid mode: {}", mode_str))
}
