mod request;
mod response;

use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

use rs_concurrency;
use rs_concurrency::ThreadPool;

pub fn initialise_connection() {
    let args: Vec<String> = env::args().collect();
    let port: &String = &args[1];
    let loopback: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket: SocketAddrV4 = SocketAddrV4::new(loopback, port.parse().unwrap());
    let listener: TcpListener = TcpListener::bind(socket).unwrap();

    println!("[Server]: Rustic Server is listening on port {}", port);

    handle_connection(&listener);
}

fn handle_connection(listener: &TcpListener){
    let pool_size: usize = 100;
    let pool: ThreadPool = rs_concurrency::ThreadPool::new(pool_size);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[Server]: New client: {:?}", stream);

                pool.execute( || {
                    request::handle_request(stream);
                });
            }
            Err(e) => println!("[Server]: Unable to get the new client: {:?}", e),
        }
    }
}
