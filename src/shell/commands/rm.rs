use std::fs;
use std::path::Path;

pub fn rm(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: rm <file>...");
        return;
    }

    for file in args {
        let path = Path::new(file);
        if let Err(e) = fs::remove_file(&path) {
            eprintln!("rm: cannot remove '{}': {}", file, e);
        }
    }
}
