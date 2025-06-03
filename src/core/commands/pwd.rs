use std::env;

// Print the current working directory
pub fn print_working_directory(_args: Vec<&str>) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("Error retrieving current directory: {}", e),
    }
}
