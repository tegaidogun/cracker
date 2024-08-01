use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn cat(args: Vec<&str>) {
    if args.is_empty() {
        eprintln!("usage: cat <file>...");
        return;
    }

    for file_name in args {
        let path = Path::new(file_name);
        match File::open(&path) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    eprintln!("cat: {}: {}", file_name, e);
                } else {
                    print!("{}", contents);
                }
            }
            Err(e) => eprintln!("cat: {}: {}", file_name, e),
        }
    }
}
