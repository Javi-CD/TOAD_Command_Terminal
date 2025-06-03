use std::fs;

// List the contents of the current working directory
pub fn list_directory(_args: Vec<&str>) {
    // Attempt to read the current directory
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                // Handle each entry in the directory
                match entry {
                    Ok(e) => {
                        // Get the file type and name, then print it
                        if let Ok(file_type) = e.file_type() {
                            let name: String = e.file_name().to_string_lossy().into_owned();

                            if file_type.is_dir() {
                                println!("{}/", name);
                            } else {
                                println!("{}", name);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}
