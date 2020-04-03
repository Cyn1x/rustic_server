use std::net::TcpStream;
use std::io::{Read, Write};

use super::response;

pub fn handle_request(mut stream: TcpStream) {
    let client: u16 = stream.peer_addr().unwrap().port();
    let mut buffer:[u8; 1024] = [0; 1024];

    loop {
        stream.read(&mut buffer).unwrap();

        let request: &str = process_request(&buffer);
        // let response: &[u8] = response::handle_response(&buffer, request).as_bytes();

        stream.write(request.as_ref()).unwrap();
        stream.flush().unwrap();

        println!("[Client {:?}]: {}", client, String::from_utf8_lossy(&buffer[..]));

        buffer.iter_mut().for_each(| i: &mut u8 | *i = 0);
    }
}

fn process_request(buffer: &[u8; 1024]) -> &str {
    let start_game: &[u8; 10] = b"START GAME";

    if buffer.starts_with(start_game) {
        return "START\n"
    }

    "STOP"
}
