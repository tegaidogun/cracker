use std::fs;
use std::io;
use crate::shell::utils::path_utils::resolve_path;

pub fn rm(args: &[&str]) -> Result<(), String> {
    let mut force = false;
    let mut prompt_every = false;
    let mut prompt_once = false;
    let mut recursive = false;
    let mut verbose = false;

    let mut paths = Vec::new();

    for arg in args {
        match *arg {
            "-f" | "--force" => force = true,
            "-i" => prompt_every = true,
            "-I" => prompt_once = true,
            "-r" | "-R" | "--recursive" => recursive = true,
            "-v" | "--verbose" => verbose = true,
            "--" => break, // Stops option parsing
            _ => paths.push(resolve_path(arg)),
        }
    }

    if paths.is_empty() {
        return Err("rm: missing operand".into());
    }

    // Handle `-I` option: Prompt once before removing more than three files
    if prompt_once && paths.len() > 3 {
        if !confirm_removal("rm: remove more than 3 files?") {
            return Ok(());
        }
    }

    for path in paths {
        if path.exists() {
            if prompt_every && !confirm_removal(&format!("rm: remove {}?", path.display())) {
                continue;
            }

            if path.is_dir() {
                if recursive {
                    if let Err(e) = fs::remove_dir_all(&path) {
                        return Err(format!("rm: {}", e));
                    }
                } else {
                    return Err(format!("rm: cannot remove '{}': Is a directory", path.display()));
                }
            } else {
                if let Err(e) = fs::remove_file(&path) {
                    if !force {
                        return Err(format!("rm: {}", e));
                    }
                }
            }

            if verbose {
                println!("removed '{}'", path.display());
            }
        } else if !force {
            return Err(format!("rm: cannot remove '{}': No such file or directory", path.display()));
        }
    }

    Ok(())
}

fn confirm_removal(message: &str) -> bool {
    println!("{}", message);
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {}", e);
        return false;
    }
    let input = input.trim().to_lowercase();
    input.starts_with('y')
}
