//! Rustic Client
//!
//! `rs-client` is the entry point to the program.

use std::env;
use std::net::{TcpStream};

use std::io;
use std::io::{Write, Read};

use std::borrow::Cow;

use openssl::ssl::{SslConnector, SslMethod, SslStream, SslConnectorBuilder};

use rs_cryptography::bcrypt_handler;

/// The game state data
struct State {
    word_length: usize,
    word_hash: Vec<u8>,
    char_guesses: i32,
    word_guesses: i32
}

/// Initialises the client by connecting to the server. The function then calls another
/// function to handle incoming and outgoing requests.
fn main() {
    let args: Vec<String> = env::args().collect();
    let host: &String = &args[1];
    let port: &String = &args[2];
    let server: String = String::from(format!("{}:{}", host, port));

    let mut builder: SslConnectorBuilder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file("cert.pem").unwrap();
    let connector: SslConnector = builder.build();

    let stream: TcpStream = TcpStream::connect(&server).unwrap();
    let mut stream: SslStream<TcpStream> = connector.connect("localhost", stream).unwrap();
    let start_msg: &[u8] = b"START GAME\n";

    let (
        word_length, word_hash, char_guesses, word_guesses
    ) = (0, Vec::new(), 0, 0);

    let mut game_state = State {
        word_length,
        word_hash,
        char_guesses,
        word_guesses
    };

    stream.write_all(start_msg).expect("Error writing to stream");

    println!("Server transmitting hashed word, please wait...\n");

    let hash: Vec<u8> = process_request(&mut stream);

    game_state.word_hash = hash;

    let confirmation_msg = b"HASH RECEIVED\n";

    stream.write_all(confirmation_msg).expect("Error writing to stream");

    let response: Vec<u8> = process_request(&mut stream);
    let server_msg: Cow<str> = String::from_utf8_lossy(&response[..]);

    println!("[Server]: {}\n", server_msg);

    let word_length: usize = remove_crlf(&response).len();

    game_state.word_length = word_length;

    handle_request(&mut stream, &mut game_state);

    println!("Exiting program.\n");

    stream.shutdown().expect("Stream shutdown call failed");
}

/// Handles incoming transmissions and calls a function to transmit data to the server.
fn handle_request(stream: &mut SslStream<TcpStream>, game_state: &mut State) {
    loop {
        handle_response(stream, game_state);

        let response: Vec<u8> = process_request(stream);
        let server_msg: Cow<str> = String::from_utf8_lossy(&response[..]);

        println!("[Server]: {}\n", server_msg);

        if server_msg.contains("GAME OVER") {
            word_integrity(&game_state.word_hash, &response);
            score_integrity(&game_state, &response);
            break;
        }
    }
}

/// Receives and processes data from the socket. Removes any padded space as it appears the stream
/// is peeking a larger number of bytes than required for the data due to the size of the
/// encrypted data over TLS.
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
fn handle_response(stream: &mut SslStream<TcpStream>, state: &mut State) {
    loop {
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if valid_response(&input) {
                    let buffer: &[u8] = input.as_bytes();
                    stream.write(buffer).expect("Error writing to stream");

                    let word: Vec<u8> = remove_crlf(&buffer);

                    if word.len() > 1 { state.word_guesses += 1; }
                    else { state.char_guesses += 1; }

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
fn word_integrity(hash: &Vec<u8>, buffer: &Vec<u8>) {
    let word: Vec<u8> = remove_crlf(&buffer);

    let word_str: Cow<str> = String::from_utf8_lossy(&word[..]);
    let hash_str: Cow<str> = String::from_utf8_lossy(&hash[..]);

    println!("[Client]: Verifying guessed word with the original hashed word, please wait...\n");

    let hashes_equal: bool = bcrypt_handler::verify_data(&word_str, &hash_str);

    if hashes_equal {
        println!("[Client]: Starting word has not changed during the game. Integrity has been preserved.\n");
        return
    }

    println!("[Client]: Starting word has not changed during the game. Integrity has been compromised.");
}

/// Checks the score with the final score sent by the server to ensure it has not been incorrectly
/// calculating it throughout the connection.
fn score_integrity(game_state: &State, buffer: &[u8]) {
    let client_score: i32 = 10 * (game_state.word_length as i32) - 2 * (game_state.char_guesses)
        - (game_state.word_guesses);

    let filter_score: Vec<u8> = buffer.iter()
        .filter(|&x| x.is_ascii_digit())
        .cloned()
        .collect::<Vec<_>>();
    let server_score = String::from_utf8_lossy(&filter_score[..]).parse::<i32>().unwrap();

    println!("[Client]: The locally calculated score is: {}\n", client_score);

    println!("[Client]: The server score is {:?}\n", server_score);

    if client_score == server_score {
        println!("[Client]: Local and server scores are the same. Integrity has been preserved.\n");
        return
    }

    println!("[Client]: Scores have been incorrectly handled by the server. Integrity has been compromised.");
}

/// Splits the buffer at the CR on Windows and the NL on UNIX-based systems. Returns a buffer with
/// all bytes excluding the split data.
fn remove_crlf(buffer: &[u8]) -> Vec<u8> {
    let word: Vec<u8> = buffer.split(|&b | b == 13 || b == 10)
        .next().unwrap().to_vec();

    word
}
