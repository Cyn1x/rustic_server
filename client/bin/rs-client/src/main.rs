//! Rustic Client
//!
//! `rs-client` is the entry point to the program.

use std::env;
use std::net::{TcpStream, Shutdown};
use std::io;
use std::io::{Write, Read};
use std::borrow::Cow;

/// Initialises the client by connecting to the server. The function then calls another
/// function to handle incoming and outgoing requests.
fn main() {
    let args: Vec<String> = env::args().collect();
    let host: &String = &args[1];
    let port: &String = &args[2];
    let server: String = String::from(format!("{}:{}", host, port));
    let mut stream = TcpStream::connect(server)
        .expect("Error connecting to server");

    stream.write(b"START GAME").expect("Error writing to stream");

    handle_request(&mut stream);

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}

/// Handles incoming transmissions and calls a function to transmit data to the server.
fn handle_request(stream: &mut TcpStream) {
    let mut peek_buffer:[u8; 1024] = [0; 1024];

    loop {
        let incoming_bytes: usize = stream.peek(&mut peek_buffer)
            .expect("Error peeking incoming bytes");
        let mut buffer: Vec<u8> = vec![0; incoming_bytes];

        stream.read(&mut buffer).expect("Error reading from stream");

        let server_msg: Cow<str> = String::from_utf8_lossy(&buffer[..]);

        println!("[Server]: {}\n", server_msg);

        if server_msg.contains("GAME OVER") { break; }

        handle_response(stream);
    }
}

/// Reads user input and transmits it to the server. The loop will only break when valid input
/// gets entered.
fn handle_response(stream: &mut TcpStream) {
    loop {
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if valid_response(&input) {
                    stream.write(&input.as_bytes()).expect("Error writing to stream");
                    break;
                }
            }
            Err(e) => println!("error: {}", e),
        }
    }
}

/// Determines whether the input is a valid char. Returns true or false
fn valid_response(input: &String) -> bool {
    for c in input.chars() {
        if !((((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) || (c == '\r' || c =='\n')) && input.len() > 2) {
            println!("[Client]: Invalid input. Only characters A-Z and a-z are permitted.");
            return false
        }
    }
    true
}
