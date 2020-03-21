use std::net::{Shutdown, TcpStream};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Error connecting to server.");

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}
