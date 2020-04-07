use std::env;
use std::net::{TcpStream, Shutdown};
use std::io;
use std::io::{Write, Read};
use std::borrow::Cow;

fn main() {
    let args: Vec<String> = env::args().collect();
    let host: &String = &args[1];
    let port: &String = &args[2];
    let server: String = String::from(format!("{}:{}", host, port));
    let mut stream = TcpStream::connect(server).unwrap();

    stream.write(b"START GAME").unwrap();

    handle_request(&mut stream);

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}

fn handle_request(stream: &mut TcpStream) {
    let mut peek_buffer:[u8; 1024] = [0; 1024];

    loop {
        let incoming_bytes: usize = stream.peek(&mut peek_buffer).unwrap();
        let mut buffer: Vec<u8> = vec![0; incoming_bytes];

        stream.read(&mut buffer).unwrap();

        let server_msg: Cow<str> = String::from_utf8_lossy(&buffer[..]);

        println!("[Server]: {}\n", server_msg);

        if server_msg.contains("GAME OVER") { break; }

        handle_response(stream);
    }
}

fn handle_response(stream: &mut TcpStream) {
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            stream.write(&input.as_bytes()).unwrap();
        }
        Err(error) => println!("error: {}", error),
    }
}
