use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use filetime::{self, FileTime};

pub fn touch(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: touch <file>...");
        return;
    }

    for file in args {
        let path = Path::new(file);
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
