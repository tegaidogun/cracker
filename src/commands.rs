use std::process::{Command, Stdio};

pub fn execute(input: &str) -> bool {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    match Command::new(command)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
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
