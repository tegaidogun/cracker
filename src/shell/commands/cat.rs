use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn cat(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: cat <file>...");
        return;
    }

    for file_name in args {
        let path = resolve_path(file_name);
        match File::open(&path) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    eprintln!("cat: {}: {}", file_name, e);
                } else {
                    print!("{}", contents);
                }
            }
            Err(e) => eprintln!("cat: {}: {}", file_name, e),
        }
    }
}

fn resolve_path(file_name: &str) -> PathBuf {
    if file_name.starts_with('~') {
        if let Some(home_dir) = get_home_dir() {
            return PathBuf::from(home_dir).join(file_name.trim_start_matches('~'));
        }
    }
    PathBuf::from(file_name)
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
