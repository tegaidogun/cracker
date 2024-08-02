use std::env;
use std::path::PathBuf;

pub fn echo(args: Vec<&str>) {
    if args.is_empty() {
        println!();
    } else {
        let expanded_args: Vec<String> = args.iter().map(|&arg| expand_argument(arg)).collect();
        println!("{}", expanded_args.join(" "));
    }
}

fn expand_argument(arg: &str) -> String {
    if arg.starts_with('~') {
        if let Some(home_dir) = get_home_dir() {
            return home_dir.join(arg.trim_start_matches('~')).to_string_lossy().to_string();
        }
    } else if arg.starts_with('$') {
        let var_name = &arg[1..];
        if let Ok(var_value) = env::var(var_name) {
            return var_value;
        }
    }
    arg.to_string()
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
