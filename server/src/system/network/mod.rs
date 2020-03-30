mod server {
    use std::env;
    use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
    use std::io::prelude::*;
    use std::thread;

    pub fn initialise_connection() {
        let args: Vec<String> = env::args().collect();
        let port: &String = &args[1];
        let loopback: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
        let socket: SocketAddrV4 = SocketAddrV4::new(loopback, port.parse().unwrap());
        let listener: TcpListener = TcpListener::bind(socket).unwrap();

        println!("Rustic Server is listening on port {}", port);

        handle_connection(listener);
    }

    fn handle_connection(listener: TcpListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client: {:?}", stream);
                    thread::spawn(|| { handle_request(stream); });
                }
                Err(e) => println!("Unable to get the new client: {:?}", e),
            }
        }
    }

    fn handle_request(mut stream: TcpStream) {
        let mut buffer:[u8; 1024] = [0; 1024];

        loop {
            stream.read(&mut buffer).unwrap();
            stream.write(b"Received").unwrap();
            stream.flush().unwrap();

            println!("[Client]: {}", String::from_utf8_lossy(&buffer[..]));
        }
    }
}

pub fn create_connection() { server::initialise_connection(); }
