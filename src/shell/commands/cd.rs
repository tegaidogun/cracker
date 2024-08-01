use std::env;
use std::path::Path;
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
    let path = Path::new(dir);
    env::set_current_dir(&path)
}
