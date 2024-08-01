use std::fs;
use std::path::Path;

pub fn mkdir(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: mkdir <directory>...");
        return;
    }

    for dir in args {
        let path = Path::new(dir);
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
}
