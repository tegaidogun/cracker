use std::{env, fs};
use std::io::{self};
use std::path::PathBuf;

pub fn cp(args: Vec<&str>) {
    if args.len() != 2 {
        eprintln!("usage: cp <source> <destination>");
        return;
    }

    let src = args[0];
    let dst = args[1];
    match copy_item(src, dst) {
        Ok(_) => {}
        Err(e) => eprintln!("cp: {}: {}", src, e),
    }
}

fn copy_item(src: &str, dst: &str) -> io::Result<()> {
    let src_path = resolve_path(src);
    let dst_path = resolve_path(dst);

    if src_path.is_dir() {
        fs::create_dir_all(&dst_path)?;
        for entry in fs::read_dir(&src_path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let src_file_path = entry.path();
            let dst_file_path = dst_path.join(file_name);
            copy_item(&src_file_path.to_string_lossy(), &dst_file_path.to_string_lossy())?;
        }
    } else {
        fs::copy(&src_path, &dst_path)?;
    }
    Ok(())
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
