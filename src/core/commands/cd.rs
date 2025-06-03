use std::env;
use std::path::Path;

// Change the current working directory to the given path
pub fn change_directory(args: Vec<&str>) {
    if args.is_empty() {
        println!("Usage: cd <path>");
        return;
    }

    let path = Path::new(args[0]);

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("Error changing directory: {}", e);
    }
}
