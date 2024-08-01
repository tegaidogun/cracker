use std::env;
use std::path::Path;
use std::io;

pub fn handle_builtin(command: &str, args: &[String]) -> bool {
    match command {
        "cd" => {
            change_directory(args);
            true
        }
        "exit" => {
            std::process::exit(0);
        }
        "help" => {
            super::help::show_help();
            true
        }
        "echo" => {
            echo(args);
            true
        }
        "pwd" => {
            print_working_directory();
            true
        }
        _ => false,
    }
}

fn change_directory(args: &[String]) {
    let dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].clone()
    };
    if let Err(e) = env::set_current_dir(Path::new(&dir)) {
        eprintln!("cracker: cd: {}: {}", dir, e);
    }
}

fn echo(args: &[String]) {
    println!("{}", args.join(" "));
}

fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("cracker: pwd: {}", e),
    }
}
