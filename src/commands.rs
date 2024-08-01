use std::process::{Command, Stdio};

pub fn execute(command: &str, args: &[String]) -> bool {
    println!("Executing command: {} with args: {:?}", command, args); // Debug print

    // Attempt to use a full path to a known executable for testing
    let full_path_command = "C:\\Windows\\System32\\cmd.exe";
    let new_args = ["/C".to_string(), command.to_string()].iter().cloned().chain(args.iter().cloned()).collect::<Vec<String>>();

    let status = Command::new(full_path_command)
        .args(&new_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) => {
            if !status.success() {
                eprintln!("cracker: command failed with status: {}", status);
            }
            true
        }
        Err(e) => {
            eprintln!("cracker: command not found: {}", e);
            false
        }
    }
}
