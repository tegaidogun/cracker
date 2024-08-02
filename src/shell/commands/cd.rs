use std::env;
use std::path::{Path, PathBuf};
use std::io::Error;

pub fn cd(args: Vec<&str>) {
    if args.len() != 1 {
        eprintln!("usage: cd <directory>");
        return;
    }

    let new_dir = args[0];
    if let Err(e) = change_directory(new_dir) {
        eprintln!("cd: {}: {}", new_dir, e);
    }
}

fn change_directory(dir: &str) -> Result<(), Error> {
    let path = if dir.starts_with('~') {
        if let Some(home_dir) = get_home_dir() {
            PathBuf::from(home_dir).join(dir.trim_start_matches('~'))
        } else {
            Path::new(dir).to_path_buf()
        }
    } else {
        Path::new(dir).to_path_buf()
    };
    env::set_current_dir(&path)
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
