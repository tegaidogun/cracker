use std::fs;
use std::path::{PathBuf};
use std::env;

pub fn mv(args: Vec<&str>) {
    if args.len() != 2 {
        eprintln!("usage: mv <source> <destination>");
        return;
    }

    let src = resolve_path(args[0]);
    let dst = resolve_path(args[1]);

    match fs::rename(src, dst) {
        Ok(_) => {}
        Err(e) => eprintln!("mv: cannot move '{}' to '{}': {}", args[0], args[1], e),
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
