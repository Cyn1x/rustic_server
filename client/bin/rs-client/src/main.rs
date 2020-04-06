use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io;
use std::io::{Write, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let host: &String = &args[1];
    let port: &String = &args[2];
    let server = String::from(format!("{}:{}", host, port));
    let mut stream = TcpStream::connect(server).unwrap();
    let mut peek_buffer:[u8; 1024] = [0; 1024];
    let mut input = String::new();

    stream.write(b"START GAME").unwrap();

    loop {
        let incoming_bytes: usize = stream.peek(&mut peek_buffer).unwrap();
        let mut buffer: Vec<u8> = vec![0; incoming_bytes];

        stream.read(&mut buffer).unwrap();

        let server_msg = String::from_utf8_lossy(&buffer[..]);

        println!("[Server]: {}\n", server_msg);

        if server_msg.contains("GAME OVER") { break; }

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                stream.write(&input.as_bytes()).unwrap();
            }
            Err(error) => println!("error: {}", error),
        }
        &input.clear();

    }

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}
