mod commands;

use std::io::{self, Write};

fn main() {
    println!("Welcome to the Toad terminal, write \"Help\" to see the available commands ..");

    loop {
        print!("Toad> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            continue; // Skip empty input
        };

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..].to_vec();

        match command.to_lowercase().as_str() {
            "cd" => commands::cd::change_directory(args.clone()),
            "ls" => commands::ls::list_directory(args.clone()),
            "clear" => commands::clear::clear_screen(args.clone()),
            "echo" => commands::echo::echo(args.clone()),
            "pwd" => commands::pwd::print_working_directory(args.clone()),
            "help" => commands::help::display_help(args.clone()),
            "exit" => {
                println!("Exiting Toad terminal...");
                break;
            }
            _ => println!("Unknown command: {}. Type 'help' for a list of available commands.", command),
        }
    }
}
