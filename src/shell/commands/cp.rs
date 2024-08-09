use crate::shell::utils::path_utils::resolve_path;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

pub fn cp(args: &[&str]) -> Result<(), String> {
    let mut recursive = false;
    let mut force = false;
    let mut interactive = false;
    let mut preserve = false;
    let mut verbose = false;
    let mut backup = false;

    let mut sources = Vec::new();
    let mut destination: Option<&str> = None;

    for arg in args {
        match *arg {
            "-R" | "-r" => recursive = true,
            "-f" => force = true,
            "-i" => interactive = true,
            "-p" => preserve = true,
            "-v" => verbose = true,
            "-b" => backup = true,
            _ => {
                if destination.is_none() {
                    sources.push(arg);
                } else {
                    destination = Some(arg);
                }
            }
        }
    }

    if sources.is_empty() {
        return Err("cp: missing file operand".into());
    }

    if destination.is_none() {
        return Err("cp: missing destination file operand after source".into());
    }

    let destination = resolve_path(destination.unwrap());

    for source in sources {
        let source_path = resolve_path(source);

        if source_path.is_dir() && !recursive {
            return Err(format!("cp: -r not specified; omitting directory '{}'", source));
        }

        let mut dest_path = destination.clone();
        if destination.is_dir() {
            dest_path = destination.join(source_path.file_name().unwrap());
        }

        if dest_path.exists() {
            if interactive {
                print!("cp: overwrite '{}'? ", dest_path.display());
                io::stdout().flush().unwrap();
                let mut response = String::new();
                io::stdin().read_line(&mut response).unwrap();
                if !response.starts_with('y') {
                    continue;
                }
            } else if force {
                fs::remove_file(&dest_path).map_err(|e| format!("cp: {}", e))?;
            } else if backup {
                let backup_path = format!("{}~", dest_path.display());
                fs::rename(&dest_path, backup_path).map_err(|e| format!("cp: {}", e))?;
            }
        }

        if recursive && source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path, preserve, verbose)?;
        } else {
            copy_file(&source_path, &dest_path, preserve, verbose)?;
        }
    }

    Ok(())
}

fn copy_file(source: &Path, destination: &Path, preserve: bool, verbose: bool) -> Result<(), String> {
    if verbose {
        println!("'{}' -> '{}'", source.display(), destination.display());
    }

    fs::copy(source, destination).map_err(|e| format!("cp: {}", e))?;

    if preserve {
        preserve_metadata(source, destination)?;
    }

    Ok(())
}

fn copy_dir_recursive(source: &Path, destination: &Path, preserve: bool, verbose: bool) -> Result<(), String> {
    if verbose {
        println!("'{}' -> '{}'", source.display(), destination.display());
    }

    fs::create_dir_all(destination).map_err(|e| format!("cp: {}", e))?;

    for entry in fs::read_dir(source).map_err(|e| format!("cp: {}", e))? {
        let entry = entry.map_err(|e| format!("cp: {}", e))?;
        let entry_path = entry.path();
        let dest_path = destination.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path, preserve, verbose)?;
        } else {
            copy_file(&entry_path, &dest_path, preserve, verbose)?;
        }
    }

    Ok(())
}

fn preserve_metadata(source: &Path, destination: &Path) -> Result<(), String> {
    let metadata = fs::metadata(source).map_err(|e| format!("cp: {}", e))?;
    fs::set_permissions(destination, metadata.permissions()).map_err(|e| format!("cp: {}", e))?;
    // Preserving other metadata like timestamps would require platform-specific code.
    Ok(())
}
