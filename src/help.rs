pub fn show_help() {
    println!("Cracker Shell - Available commands:");
    println!("cd <dir>   - Change directory");
    println!("exit       - Exit the shell");
    println!("help       - Show this help message");
    println!("echo <text> - Display a line of text");
    println!("pwd        - Print the current working directory");
    println!("ls         - List directory contents");
    println!("cat <file> - Display file contents");
    println!("grep <pattern> <file> - Search for patterns in files");
    println!("touch <file> - Create an empty file");
    println!("rm <file>   - Remove files or directories");
    println!("<command>  - Execute an external command");
    println!("For more information on a specific command, use 'man <command>' (if available).");
}
