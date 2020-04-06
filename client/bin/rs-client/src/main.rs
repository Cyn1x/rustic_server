use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io;
use std::io::{Write, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let host: &String = &args[1];
    let port: &String = &args[2];
    let server: String = String::from(format!("{}:{}", host, port));
    let mut stream = TcpStream::connect(server).unwrap();

    stream.write(b"START GAME").unwrap();

    server_recv(&mut stream);

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}

fn server_recv(stream: &mut TcpStream) {
    let mut peek_buffer:[u8; 1024] = [0; 1024];

    loop {
        let incoming_bytes: usize = stream.peek(&mut peek_buffer).unwrap();
        let mut buffer: Vec<u8> = vec![0; incoming_bytes];

        stream.read(&mut buffer).unwrap();

        let server_msg = String::from_utf8_lossy(&buffer[..]);

        println!("[Server]: {}\n", server_msg);

        if server_msg.contains("GAME OVER") { break; }

        server_send(stream);
    }
}

fn server_send(stream: &mut TcpStream) {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            stream.write(&input.as_bytes()).unwrap();
        }
        Err(error) => println!("error: {}", error),
    }
}
