// Clears the terminal screen by printing ANSI escape codes
pub fn clear_screen(_args: Vec<&str>) {
    // Print the ANSI escape code to clear the screen
    println!("\x1B[2J\x1B[1;1H");
}
