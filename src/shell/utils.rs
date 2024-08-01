use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

#[allow(dead_code)]
/// Prints an error message to stderr.
pub fn print_error(message: &str) {
    eprintln!("error: {}", message);
}

#[allow(dead_code)]
/// Reads the contents of a file and returns it as a String.
pub fn read_file(file_path: &Path) -> Result<String, io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[allow(dead_code)]
/// Writes a string to a file, overwriting its contents.
pub fn write_file(file_path: &Path, content: &str) -> Result<(), io::Error> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
/// Appends a string to a file.
pub fn append_to_file(file_path: &Path, content: &str) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
/// Checks if a path exists and is a directory.
pub fn is_directory(path: &Path) -> bool {
    path.is_dir()
}

#[allow(dead_code)]
/// Checks if a path exists and is a file.
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

#[allow(dead_code)]
/// Gets the current working directory as a PathBuf.
pub fn current_dir() -> Result<std::path::PathBuf, io::Error> {
    std::env::current_dir()
}
