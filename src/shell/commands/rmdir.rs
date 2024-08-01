use std::fs;
use std::path::Path;

pub fn rmdir(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: rmdir <directory>...");
        return;
    }

    for dir in args {
        let path = Path::new(dir);
        if let Err(e) = fs::remove_dir(&path) {
            eprintln!("rmdir: cannot remove '{}': {}", dir, e);
        }
    }
}
