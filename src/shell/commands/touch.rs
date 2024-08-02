use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};
use filetime::{self, FileTime};
use std::env;

pub fn touch(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: touch <file>...");
        return;
    }

    for file in args {
        let path = resolve_path(file);
        if let Err(e) = create_or_update_file(&path) {
            eprintln!("touch: cannot touch '{}': {}", file, e);
        }
    }
}

fn create_or_update_file(path: &Path) -> io::Result<()> {
    if path.exists() {
        let _ = filetime::set_file_mtime(path, FileTime::now());
    } else {
        OpenOptions::new().write(true).create(true).open(path)?;
    }
    Ok(())
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
