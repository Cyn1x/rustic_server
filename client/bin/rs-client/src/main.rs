use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io;
use std::io::{Write, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port: &String = &args[1];
    let server = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(server, port.parse().unwrap());
    let mut stream = TcpStream::connect(socket).unwrap();
    let mut peek_buffer:[u8; 1024] = [0; 1024];
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                stream.write(&input.as_bytes()).unwrap();

                let incoming_bytes: usize = stream.peek(&mut peek_buffer).unwrap();
                let mut buffer: Vec<u8> = vec![0; incoming_bytes];

                stream.read(&mut buffer).unwrap();

                println!("[Server]: {}\n", String::from_utf8_lossy(&buffer[..]));
            }
            Err(error) => println!("error: {}", error),
        }
        &input.clear();

    }

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}
