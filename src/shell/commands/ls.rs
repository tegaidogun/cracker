use std::env;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

pub fn ls(args: Vec<&str>) {
    let target_dir = if args.is_empty() { "." } else { args[0] };
    let path = resolve_path(target_dir);
    
    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => print_entry(dir_entry),
                    Err(e) => eprintln!("ls: error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("ls: cannot access '{}': {}", target_dir, e),
    }
}

fn print_entry(entry: DirEntry) {
    if let Ok(file_name) = entry.file_name().into_string() {
        println!("{}", file_name);
    }
}

fn resolve_path(path_str: &str) -> PathBuf {
    if path_str.starts_with('~') {
        if let Some(home_dir) = get_home_dir() {
            return PathBuf::from(home_dir).join(path_str.trim_start_matches('~'));
        }
    }
    PathBuf::from(path_str)
}

fn get_home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        env::var("HOME").ok().map(PathBuf::from)
    }

    #[cfg(windows)]
    {
        env::var("USERPROFILE").ok().map(PathBuf::from)
    }
}
