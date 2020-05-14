//! Rustic Client
//!
//! `rs-client` is the entry point to the program.

use std::env;
use std::net::{TcpStream};

use std::io;
use std::io::{Write, Read};

use std::borrow::Cow;

use openssl::ssl::{SslConnector, SslMethod, SslStream};

use rs_cryptography::bcrypt_handler;

/// Initialises the client by connecting to the server. The function then calls another
/// function to handle incoming and outgoing requests.
fn main() {
    let args: Vec<String> = env::args().collect();
    let host: &String = &args[1];
    let port: &String = &args[2];
    let server: String = String::from(format!("{}:{}", host, port));

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file("cert.pem").unwrap();
    let connector = builder.build();

    let stream = TcpStream::connect(&server).unwrap();
    let mut stream = connector.connect("localhost", stream).unwrap();
    let start_msg = b"START GAME\n";

    stream.write_all(start_msg).expect("Error writing to stream");

    println!("Server transmitting hashed word, please wait...\n");

    let hash: Vec<u8> = process_request(&mut stream);

    let confirmation_msg = b"HASH RECEIVED\n";

    stream.write_all(confirmation_msg).expect("Error writing to stream");

    handle_request(&mut stream, &hash);

    println!("Exiting program.\n");
    stream.shutdown().expect("Stream shutdown call failed");
}

/// Handles incoming transmissions and calls a function to transmit data to the server.
fn handle_request(stream: &mut SslStream<TcpStream>, hash: &Vec<u8>) {

    loop {
        let response: Vec<u8> = process_request(stream);
        let server_msg = String::from_utf8_lossy(&response[..]);

        println!("[Server]: {}\n", server_msg);

        if server_msg.contains("GAME OVER") {
            integrity_check(hash, &response);
            break;
        }

        handle_response(stream);
    }
}

fn process_request(stream: &mut SslStream<TcpStream>) -> Vec<u8> {
    let mut peek_buffer:[u8; 1024] = [0; 1024];

    let incoming_bytes: usize = stream.get_mut().peek(&mut peek_buffer)
        .expect("Error peeking incoming bytes");
    let mut buffer: Vec<u8> = vec![0; incoming_bytes];

    stream.read(&mut buffer).expect("Error reading from stream");

    let split_buffer: Vec<u8> = buffer.split(|&b | b == 0)
        .next().unwrap().to_vec();

    split_buffer
}

/// Reads user input and transmits it to the server. The loop will only break when valid input
/// gets entered.
fn handle_response(stream: &mut SslStream<TcpStream>) {
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
        if !((((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) || (c == '\r' || c =='\n')) && input.len() > 1) {
            println!("[Client]: Invalid input. Only characters A-Z and a-z are permitted.");
            return false
        }
    }
    true
}

/// Verifies the hashed word from the beginning phase of the game, with the hash of the word
/// at the end of the game.
fn integrity_check(hash: &Vec<u8>, buffer: &Vec<u8>) {
    let word: Vec<u8> = buffer.split(|&b | b == 13 || b == 10)
        .next().unwrap().to_vec();

    let word_str: Cow<str> = String::from_utf8_lossy(&word[..]);
    let hash_str: Cow<str> = String::from_utf8_lossy(&hash[..]);

    println!("Verifying guessed word with the original hashed word, please wait...\n");

    let hashes_equal: bool = bcrypt_handler::verify_data(&word_str, &hash_str);

    if hashes_equal {
        println!("Starting word has not changed during the game. Integrity has been preserved.\n");
        return
    }

    println!("Starting word has not changed during the game. Integrity has been compromised.");
}
