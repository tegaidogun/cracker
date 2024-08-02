mod shell;

use std::env;
use crate::shell::start_shell;

fn main() {
    let args: Vec<String> = env::args().collect();
    let show_pwd = args.iter().any(|arg| arg == "-d");
    start_shell(show_pwd);
}
