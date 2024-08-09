use crate::shell::utils::path_utils::resolve_path;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn mv(args: &[&str]) -> Result<(), String> {
    let mut backup = false;
    let mut force = false;
    let mut interactive = false;
    let mut update = false;
    let mut verbose = false;
    let mut suffix = "~";
    let mut source_paths = Vec::new();
    let mut destination = None;

    let mut iter = args.iter().peekable();

    while let Some(&arg) = iter.next() {
        match arg {
            "-b" | "--backup" => backup = true,
            "-f" | "--force" => force = true,
            "-i" | "--interactive" => interactive = true,
            "-u" | "--update" => update = true,
            "-v" | "--verbose" => verbose = true,
            "-S" | "--suffix" => {
                if let Some(&next_arg) = iter.peek() {
                    suffix = next_arg;
                    iter.next();
                } else {
                    return Err("mv: missing argument for '--suffix'".into());
                }
            }
            _ => {
                let path = resolve_path(arg);
                if iter.peek().is_none() {
                    destination = Some(path);
                } else {
                    source_paths.push(path);
                }
            }
        }
    }

    if source_paths.is_empty() || destination.is_none() {
        return Err("mv: missing file operand".into());
    }

    let destination = destination.unwrap();

    if source_paths.len() > 1 && !destination.is_dir() {
        return Err(format!("mv: target '{}' is not a directory", destination.display()));
    }

    for source in source_paths {
        let target = if destination.is_dir() {
            destination.join(source.file_name().unwrap())
        } else {
            destination.clone()
        };

        if update && target.exists() && !should_update(&source, &target) {
            continue;
        }

        if backup && target.exists() {
            let backup_target = target.with_extension(suffix);
            fs::rename(&target, &backup_target)
                .map_err(|e| format!("mv: cannot backup '{}': {}", target.display(), e))?;
        }

        if force || !target.exists() {
            fs::rename(&source, &target)
                .map_err(|e| format!("mv: cannot move '{}': {}", source.display(), e))?;
        } else if interactive {
            print!("mv: overwrite '{}'? ", target.display());
            io::stdout().flush().unwrap();

            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();

            if response.trim().to_lowercase().starts_with('y') {
                fs::rename(&source, &target)
                    .map_err(|e| format!("mv: cannot move '{}': {}", source.display(), e))?;
            }
        }

        if verbose {
            println!("'{}' -> '{}'", source.display(), target.display());
        }
    }

    Ok(())
}

fn should_update(source: &Path, target: &Path) -> bool {
    let source_metadata = fs::metadata(source).ok();
    let target_metadata = fs::metadata(target).ok();

    match (source_metadata, target_metadata) {
        (Some(src_meta), Some(dst_meta)) => {
            src_meta.modified().ok() > dst_meta.modified().ok()
        }
        _ => true,
    }
}
