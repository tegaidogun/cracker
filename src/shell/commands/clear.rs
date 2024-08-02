#[cfg(unix)]
use std::io::{self, Write};

#[cfg(target_os = "windows")]
pub fn clear() {
    let _ = std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status();
}

#[cfg(not(target_os = "windows"))]
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush(); // Ensure output is flushed on Unix-like systems
}
