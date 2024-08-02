use std::env;
use std::path::PathBuf;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            let display_path = if let Some(home_dir) = get_home_dir() {
                if path.starts_with(&home_dir) {
                    format!("~{}", path.strip_prefix(home_dir).unwrap().display())
                } else {
                    path.display().to_string()
                }
            } else {
                path.display().to_string()
            };
            println!("{}", display_path);
        }
        Err(e) => eprintln!("pwd: {}", e),
    }
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
