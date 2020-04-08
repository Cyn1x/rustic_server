use std::net::TcpStream;
use std::io::Read;
use std::borrow::Cow;

use rs_handler::Handler;

use super::response;

/// Handles a client by sending the incoming transmission to `rs-handler`, then awaits the
/// response to transmit back to the client.
pub fn handle_request(mut stream: TcpStream) {
    let client: u16 = stream.peer_addr().unwrap().port();
    let mut peek_buffer: [u8; 1024] = [0; 1024];
    let mut handler: Handler = rs_handler::Handler::new();

    loop {
        let incoming_bytes: usize = stream.peek(&mut peek_buffer)
            .expect("Error peeking incoming bytes");
        let mut buffer: Vec<u8> = vec![0; incoming_bytes];

        stream.read(&mut buffer).expect("Error reading from stream");

        let response: &[u8] = handler.handle_request(&buffer);
        response::handle_response(&stream, response);

        let server_msg: Cow<str> = String::from_utf8_lossy(&response[..]);
        let client_msg: Cow<str> = String::from_utf8_lossy(&buffer[..]);
        println!("[Client {:?}]: {}", client, client_msg);

        if server_msg.contains("GAME OVER") { break; }
    }
}
