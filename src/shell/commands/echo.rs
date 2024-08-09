use crate::shell::utils::path_utils::resolve_path;

pub fn echo(args: &[&str]) {
    let expanded_args: Vec<String> = args.iter().map(|arg| resolve_argument(arg)).collect();
    println!("{}", expanded_args.join(" "));
}

fn resolve_argument(arg: &str) -> String {
    if arg.starts_with('~') || arg.starts_with('$') {
        let resolved_path = resolve_path(arg);
        resolved_path.display().to_string()
    } else {
        arg.to_string()
    }
}
