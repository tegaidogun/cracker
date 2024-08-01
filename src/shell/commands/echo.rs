pub fn echo(args: Vec<&str>) {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
}
