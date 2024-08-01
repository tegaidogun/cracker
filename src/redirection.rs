use std::fs::File;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

pub fn execute_with_redirection(command: &str, args: &[&str], input_redir: Option<&str>, output_redir: Option<&str>, append: bool) {
    let mut command_process = Command::new(command);

    if let Some(input_file) = input_redir {
        let file = File::open(input_file).unwrap_or_else(|_| {
            eprintln!("cracker: no such file or directory: {}", input_file);
            std::process::exit(1);
        });
        command_process.stdin(Stdio::from(file));
    }

    if let Some(output_file) = output_redir {
        let file = if append {
            OpenOptions::new().write(true).create(true).append(true).open(output_file)
        } else {
            OpenOptions::new().write(true).create(true).truncate(true).open(output_file)
        }.unwrap_or_else(|_| {
            eprintln!("cracker: cannot write to file: {}", output_file);
            std::process::exit(1);
        });
        command_process.stdout(Stdio::from(file));
    }

    let status = command_process.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) => {
            if !status.success() {
                eprintln!("cracker: command failed with status: {}", status);
            }
        }
        Err(e) => eprintln!("cracker: command not found: {}", e),
    }
}
