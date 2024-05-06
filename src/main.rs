use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read data from the client
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Echo the received data back to the client
            match stream.write(&buffer) {
                Ok(_) => println!("Echoed back to client"),
                Err(e) => eprintln!("Error writing to stream: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading from stream: {}", e),
    }
}

fn main() -> std::io::Result<()> {
    // Bind the server to localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080...");

    // Accept incoming connections and spawn a new thread for each client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }

    Ok(())
}
