use std::process;

pub fn exit_shell(args: &[&str], last_status: i32) -> ! {
    let exit_code = if !args.is_empty() {
        args[0].parse::<i32>().unwrap_or(1) // Default to exit code 1 if parsing fails
    } else {
        last_status // Use the status of the last executed command
    };

    process::exit(exit_code);
}
