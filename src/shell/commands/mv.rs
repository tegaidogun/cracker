use crate::shell::utils::path_utils::resolve_path;
use std::fs;

pub fn mv(args: &[&str]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("mv: missing file operand".into());
    }

    let source = resolve_path(args[0]);
    let destination = resolve_path(args[1]);

    if source.exists() {
        fs::rename(&source, &destination).map_err(|e| format!("mv: cannot move '{}': {}", source.display(), e))?;
        Ok(())
    } else {
        Err(format!("mv: cannot stat '{}': No such file or directory", source.display()))
    }
}
