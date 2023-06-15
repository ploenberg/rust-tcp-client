use std::env;
use std::net::TcpStream;
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 50 * 1024 * 1024; // 50MB

fn main() {
    // Get the server IP address from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./client -s <server_ip>");
        return;
    }

    if args[1] != "-s" {
        eprintln!("Usage: ./client -s <server_ip>");
        return;
    }

    let server_address = &args[2];

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to server at {}", server_address);

            let mut buffer = [0; 1024];
            let mut total_received: usize = 0;

            loop {
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        if size == 0 {
                            break; // Connection closed
                        }

                        total_received += size;

                        // Check if 50MB has been received
                        if total_received % CHUNK_SIZE == 0 {
                            println!("Received {}MB of traffic", total_received / (1024 * 1024));
                        }

                        // Process received data here...
                        // You can modify this part to handle the received data as per your requirements.
                    }
                    Err(e) => {
                        eprintln!("Error reading from socket: {}", e);
                        break;
                    }
                }
            }

            println!("Total data received: {} bytes", total_received);
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}