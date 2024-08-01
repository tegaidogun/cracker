// main.rs or lib.rs
mod builtins;
mod commands;
mod help;
mod redirection;
mod shell;
mod utils;

fn main() {
    shell::start();
}
