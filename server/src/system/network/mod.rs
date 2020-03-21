mod request;

mod server {
    use std::io::prelude::*;
    use std::fs;
    use std::net::{TcpListener, TcpStream};

    use crate::system::network;

    pub fn initialise_connection() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer:[u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let (status_line, filename) = network::request::handle_request(&buffer);
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}

pub fn create_connection() { server::initialise_connection(); }
