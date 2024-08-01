pub fn help(args: Vec<&str>) {
    if args.is_empty() {
        print_general_help();
    } else {
        match args[0] {
            "cd" => print_cd_help(),
            "ls" => print_ls_help(),
            "cp" => print_cp_help(),
            "mkdir" => print_mkdir_help(),
            "mv" => print_mv_help(),
            "pwd" => print_pwd_help(),
            "rm" => print_rm_help(),
            "cat" => print_cat_help(),
            "rmdir" => print_rmdir_help(),
            "touch" => print_touch_help(),
            _ => eprintln!("No help available for command: {}", args[0]),
        }
    }
}

fn print_general_help() {
    println!("Usage: <command> [arguments]");
    println!("Available commands:");
    println!("  cd <directory>     Change the current directory.");
    println!("  ls [directory]     List directory contents.");
    println!("  cp <source> <destination>  Copy files or directories.");
    println!("  mkdir <directory>  Create new directories.");
    println!("  mv <source> <destination>  Move or rename files or directories.");
    println!("  pwd                Print the current working directory.");
    println!("  rm <file>          Remove files.");
    println!("  cat <file>         Display file contents.");
    println!("  rmdir <directory>  Remove empty directories.");
    println!("  touch <file>       Create an empty file or update its timestamp.");
    println!("  help [command]     Display help information for a command.");
    println!("  exit               Exit the shell.");
}

fn print_cd_help() {
    println!("cd <directory>");
    println!("  Change the current directory to <directory>.");
}

fn print_ls_help() {
    println!("ls [directory]");
    println!("  List the contents of the specified directory.");
    println!("  If no directory is specified, lists the contents of the current directory.");
}

fn print_cp_help() {
    println!("cp <source> <destination>");
    println!("  Copy the file or directory from <source> to <destination>.");
}

fn print_mkdir_help() {
    println!("mkdir <directory>");
    println!("  Create a new directory with the specified name.");
}

fn print_mv_help() {
    println!("mv <source> <destination>");
    println!("  Move or rename a file or directory.");
}

fn print_pwd_help() {
    println!("pwd");
    println!("  Print the current working directory.");
}

fn print_rm_help() {
    println!("rm <file>");
    println!("  Remove the specified file.");
}

fn print_cat_help() {
    println!("cat <file>");
    println!("  Display the contents of the specified file.");
}

fn print_rmdir_help() {
    println!("rmdir <directory>");
    println!("  Remove the specified empty directory.");
}

fn print_touch_help() {
    println!("touch <file>");
    println!("  Create an empty file or update the timestamp of an existing file.");
}
