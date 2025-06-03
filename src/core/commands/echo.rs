// Command to echo text to the console
pub fn echo(args: Vec<&str>) {
    if args.is_empty() {
        println!("Usage: echo <text>");
    } else {
        let output = args.join(" ");
        println!("{}", output);
    }
}
