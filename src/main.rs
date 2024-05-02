use chrono::{Local, DateTime};

fn main() {
    // Get the current local date and time
    let now: DateTime<Local> = Local::now();

    // Print the current date and time
    println!("Current date and time: {}", now);
}
