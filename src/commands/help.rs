// Print the list of avilable commands.
pub fn display_help(_args: Vec<&str>) {
    println!("Available commands:");
    println!("  cd <path>    - Change current directory");
    println!("  ls           - List contents of the current directory");
    println!("  pwd          - Show current working directory");
    println!("  echo <text>  - Print text to the terminal");
    println!("  clear        - Clear the terminal screen");
    println!("  help         - Show this help message");
    println!("  exit         - Exit the terminal");
}
