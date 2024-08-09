use crate::shell::utils::path_utils::resolve_path;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use chrono;

pub fn ls(args: &[&str]) -> Result<(), String> {
    let mut show_all = false;
    let mut long_format = false;
    let mut recursive = false;
    let mut human_readable = false;
    let mut directory_only = false;

    let mut paths = Vec::new();

    for arg in args {
        match *arg {
            "-a" => show_all = true,
            "-l" => long_format = true,
            "-R" => recursive = true,
            "-h" => human_readable = true,
            "-d" => directory_only = true,
            _ => paths.push(resolve_path(arg)),
        }
    }

    if paths.is_empty() {
        paths.push(PathBuf::from("."));
    }

    for path in paths {
        if directory_only || path.is_dir() {
            if recursive {
                list_dir_recursive(&path, show_all, long_format, human_readable)?;
            } else {
                list_dir(&path, show_all, long_format, human_readable)?;
            }
        } else {
            list_file(&path, long_format, human_readable)?;
        }
    }

    Ok(())
}

fn list_dir(path: &Path, show_all: bool, long_format: bool, human_readable: bool) -> Result<(), String> {
    let entries = fs::read_dir(path).map_err(|e| format!("ls: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("ls: {}", e))?;
        if show_all || !entry.file_name().to_string_lossy().starts_with('.') {
            if long_format {
                let metadata = entry.metadata().map_err(|e| format!("ls: {}", e))?;
                print_long_format_with_metadata(entry.path().as_path(), &metadata, human_readable)?;
            } else {
                print!("{}", entry.file_name().to_string_lossy());
            }
            println!();
        }
    }

    Ok(())
}


fn list_dir_recursive(path: &Path, show_all: bool, long_format: bool, human_readable: bool) -> Result<(), String> {
    if path.is_dir() {
        list_dir(path, show_all, long_format, human_readable)?;

        let entries = fs::read_dir(path).map_err(|e| format!("ls: {}", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("ls: {}", e))?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                println!("\n{}:", entry_path.display());
                list_dir_recursive(&entry_path, show_all, long_format, human_readable)?;
            }
        }
    } else {
        list_file(path, long_format, human_readable)?;
    }

    Ok(())
}

fn list_file(path: &Path, long_format: bool, human_readable: bool) -> Result<(), String> {
    if long_format {
        let metadata = fs::metadata(path).map_err(|e| format!("ls: {}", e))?;
        print_long_format_with_metadata(path, &metadata, human_readable)?;
    } else {
        println!("{}", path.display());
    }
    Ok(())
}

fn print_long_format_with_metadata(path: &Path, metadata: &fs::Metadata, human_readable: bool) -> Result<(), String> {
    let file_type = if metadata.is_dir() { "d" } else { "-" };
    let permissions = metadata.permissions().mode() & 0o777;
    let size = if human_readable {
        format_size(metadata.len())
    } else {
        format!("{}", metadata.len())
    };

    let modified_time = metadata.modified().map_err(|e| format!("ls: {}", e))?;
    let formatted_time = format_time(modified_time);

    println!(
        "{}{:o} {} {} {} {} {}",
        file_type,
        permissions,
        metadata.uid(),
        metadata.gid(),
        size,
        formatted_time,
        path.file_name().unwrap().to_string_lossy()
    );

    Ok(())
}


fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1}K", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1}M", size as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.1}G", size as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

fn format_time(time: std::time::SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    datetime.format("%b %d %H:%M").to_string()
}
