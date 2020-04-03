use std::net::TcpStream;
use std::io::{Read, Write};

use super::response;

use rs_handler::Handler;

pub fn handle_request(mut stream: TcpStream) {
    let client: u16 = stream.peer_addr().unwrap().port();
    let handler: Handler = Handler::new();
    let mut stream_buffer: [u8; 1024] = [0; 1024];

    loop {
        let incoming_bytes: usize = stream.peek(&mut stream_buffer).unwrap();
        let mut buffer: Vec<u8> = vec![0; incoming_bytes];

        stream.read(&mut buffer).unwrap();

        let request: &str = process_request(&buffer);
        let response: &[u8] = response::handle_response(&request, &handler).as_bytes();

        stream.write(response.as_ref()).unwrap();
        stream.flush().unwrap();

        println!("[Client {:?}]: {}", client, String::from_utf8_lossy(&buffer[..]));
    }
}

fn process_request(buffer: &Vec<u8>) -> &str {
    let start_game: &[u8; 10] = b"START GAME";

    if buffer.starts_with(start_game) {
        return "TEMP PLACEHOLDER: START GAME"
    }

    let test = String::from_utf8_lossy(&buffer[..]);

    println!("{}", test);
    "TEMP PLACEHOLDER: RECEIVED"
}
