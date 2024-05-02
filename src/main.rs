use std::path::PathBuf;
use rand::Rng;

fn main() {
    // Use home crate to get the user's home directory
    if let Some(home) = dirs::home_dir() {
        let mut path = PathBuf::from(home);
        path.push("your_file.txt");
        println!("Expanded path: {:?}", path);
    } else {
        println!("Could not determine home directory");
    }

    // Use rand crate to generate a random number
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", random_number);
}
