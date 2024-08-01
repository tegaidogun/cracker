use std::fs;

pub fn mv(args: Vec<&str>) {
    if args.len() != 2 {
        eprintln!("usage: mv <source> <destination>");
        return;
    }

    let src = args[0];
    let dst = args[1];

    match fs::rename(src, dst) {
        Ok(_) => {}
        Err(e) => eprintln!("mv: cannot move '{}' to '{}': {}", src, dst, e),
    }
}
