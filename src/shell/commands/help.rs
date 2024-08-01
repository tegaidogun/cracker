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
            "exit" => print_exit_help(),
            _ => eprintln!("No help available for command: {}", args[0]),
        }
    }
}

fn print_general_help() {
    println!("Usage: <command> [arguments]");
    println!("Available commands:");
    println!("  cd                 Change the current directory.");
    println!("  ls                 List directory contents.");
    println!("  cp                 Copy files or directories.");
    println!("  mkdir              Create new directories.");
    println!("  mv                 Move or rename files or directories.");
    println!("  pwd                Print the current working directory.");
    println!("  rm                 Remove files.");
    println!("  cat                Display file contents.");
    println!("  rmdir              Remove empty directories.");
    println!("  touch              Create an empty file or update its timestamp.");
    println!("  exit               Exit the shell.");
    println!("Type 'help <command>' for more information on a specific command.");
}

fn print_cd_help() {
    println!("cd <directory>");
    println!("  Change the current directory to <directory>.");
    println!("  If the directory does not exist, an error is displayed.");
    println!("Examples:");
    println!("  cd /home/user       Change to the directory /home/user.");
    println!("  cd ..               Move up one directory level.");
}

fn print_ls_help() {
    println!("ls [directory]");
    println!("  List the contents of the specified directory.");
    println!("  If no directory is specified, lists the contents of the current directory.");
    println!("Examples:");
    println!("  ls /home/user       List the contents of /home/user.");
    println!("  ls                  List the contents of the current directory.");
}

fn print_cp_help() {
    println!("cp <source> <destination>");
    println!("  Copy the file or directory from <source> to <destination>.");
    println!("  If <source> is a directory, its contents are recursively copied.");
    println!("Examples:");
    println!("  cp file.txt /tmp/               Copy file.txt to /tmp/.");
    println!("  cp -r folder /backup/folder      Recursively copy the folder to /backup.");
}

fn print_mkdir_help() {
    println!("mkdir <directory>");
    println!("  Create a new directory with the specified name.");
    println!("  If the directory already exists, an error is displayed.");
    println!("Examples:");
    println!("  mkdir new_folder     Create a directory named new_folder.");
    println!("  mkdir -p /path/to/dir Create a nested directory structure.");
}

fn print_mv_help() {
    println!("mv <source> <destination>");
    println!("  Move or rename a file or directory from <source> to <destination>.");
    println!("  If <source> and <destination> are on the same filesystem, the operation is a rename.");
    println!("Examples:");
    println!("  mv oldname.txt newname.txt        Rename the file oldname.txt to newname.txt.");
    println!("  mv file.txt /tmp/                 Move file.txt to /tmp/.");
}

fn print_pwd_help() {
    println!("pwd");
    println!("  Print the current working directory.");
    println!("  This command outputs the full path of the current directory.");
    println!("Examples:");
    println!("  pwd                               Display the path of the current directory.");
}

fn print_rm_help() {
    println!("rm <file>");
    println!("  Remove the specified file.");
    println!("  Be cautious, as this operation is irreversible.");
    println!("Examples:");
    println!("  rm file.txt                       Remove file.txt.");
    println!("  rm -r folder                      Recursively remove the folder and its contents.");
}

fn print_cat_help() {
    println!("cat <file>");
    println!("  Display the contents of the specified file.");
    println!("  Useful for viewing file contents without opening an editor.");
    println!("Examples:");
    println!("  cat file.txt                      Display the contents of file.txt.");
    println!("  cat /etc/passwd                   View the contents of the passwd file.");
}

fn print_rmdir_help() {
    println!("rmdir <directory>");
    println!("  Remove the specified empty directory.");
    println!("  The directory must be empty for this operation to succeed.");
    println!("Examples:");
    println!("  rmdir old_folder                  Remove the empty directory old_folder.");
    println!("  rmdir /tmp/empty_dir              Remove the empty directory /tmp/empty_dir.");
}

fn print_touch_help() {
    println!("touch <file>");
    println!("  Create an empty file or update the timestamp of an existing file.");
    println!("  If the file does not exist, it is created.");
    println!("Examples:");
    println!("  touch newfile.txt                 Create an empty file named newfile.txt.");
    println!("  touch existingfile.txt            Update the timestamp of existingfile.txt.");
}

fn print_exit_help() {
    println!("exit");
    println!("  Exits the current cracker shell session.");
    println!("  This command terminates the shell and returns you to the previous environment.");
    println!("Examples:");
    println!("  exit                              Exit the shell.");
}
