use std::net::TcpStream;
use std::io::Write;

/// Writes the buffer to the stream and flushes the stream.
pub fn handle_response(mut stream: &TcpStream, response: &[u8]) {
    stream.write(response).expect("Error writing to stream");
    stream.flush().expect("Error flushing stream");
}
