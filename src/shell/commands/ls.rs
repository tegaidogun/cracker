use std::fs::{self, DirEntry};
use std::path::Path;

pub fn ls(args: Vec<&str>) {
    let target_dir = if args.is_empty() { "." } else { args[0] };
    let path = Path::new(target_dir);
    
    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => print_entry(dir_entry),
                    Err(e) => eprintln!("ls: error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("ls: cannot access '{}': {}", target_dir, e),
    }
}

fn print_entry(entry: DirEntry) {
    if let Ok(file_name) = entry.file_name().into_string() {
        println!("{}", file_name);
    }
}
