use std::io;

fn main() {
    println!("Welcome to the Rust Compass!");

    loop {
        println!("Enter a direction (N, S, E, W) or 'quit' to exit:");

        let mut input = String::new();

        // Read user input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and convert to uppercase
        let direction = input.trim().to_uppercase();

        // Match user input to display the corresponding direction
        match direction.as_str() {
            "N" => println!("You are facing North"),
            "S" => println!("You are facing South"),
            "E" => println!("You are facing East"),
            "W" => println!("You are facing West"),
            "QUIT" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid direction, please try again"),
        }
    }
}
