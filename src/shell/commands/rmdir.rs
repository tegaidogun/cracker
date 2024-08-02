use std::fs;
use std::path::PathBuf;
use std::env;

pub fn rmdir(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: rmdir <directory>...");
        return;
    }

    for dir in args {
        let path = resolve_path(dir);
        if let Err(e) = fs::remove_dir(&path) {
            eprintln!("rmdir: cannot remove '{}': {}", dir, e);
        }
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
