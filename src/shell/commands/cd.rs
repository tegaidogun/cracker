use crate::shell::utils::path_utils::resolve_path;
use std::env;

pub fn cd(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err("cd: missing argument".into());
    }

    let path = resolve_path(args[0]);
    if crate::shell::utils::path_utils::path_exists_and_is_dir(&path) {
        env::set_current_dir(&path).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("cd: no such file or directory: {}", args[0]))
    }
}
