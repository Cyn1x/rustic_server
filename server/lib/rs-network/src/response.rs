use std::net::TcpStream;
use std::io::Write;

pub fn handle_response(mut stream: &TcpStream, response: &[u8]) {
    stream.write(response).unwrap();
    stream.flush().unwrap();
}
