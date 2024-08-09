use crate::shell::utils::path_utils::{resolve_path, copy_path};

pub fn cp(args: &[&str]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("cp: missing file operand".into());
    }

    let source = resolve_path(args[0]);
    let mut destination = resolve_path(args[1]);

    if source.is_file() {
        // If destination is a directory, append the source file name to the destination path
        if destination.is_dir() {
            destination = destination.join(source.file_name().unwrap());
        }
        copy_path(&source, &destination)
    } else if source.is_dir() {
        Err("cp: copying directories is not supported yet".into())
    } else {
        Err(format!("cp: cannot stat '{}': No such file or directory", args[0]))
    }
}
