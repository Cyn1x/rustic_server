use std::net::TcpStream;
use std::io::Write;
use openssl::ssl::SslStream;

/// Writes the buffer to the stream and flushes the stream.
pub fn handle_response(stream: &mut SslStream<TcpStream>, response: &[u8]) {
    stream.write(response).expect("Error writing to stream");
    stream.flush().expect("Error flushing stream");
}
