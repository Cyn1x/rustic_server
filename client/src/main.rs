use std::net::{Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io;
use std::io::{Write, Read};

fn main() {
    let server = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(server, 8080);
    let mut stream = TcpStream::connect(socket).unwrap();

    let mut buffer:[u8; 1024] = [0; 1024];
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                stream.write(&input.as_bytes());
                stream.read(&mut buffer).unwrap();

                println!("[Server]: {}", String::from_utf8_lossy(&buffer[..]));
            }
            Err(error) => println!("error: {}", error),
        }
        &input.clear();
    }

    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}
