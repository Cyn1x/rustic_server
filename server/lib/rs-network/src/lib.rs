mod request;
mod response;

use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

use rs_concurrency;
use rs_concurrency::ThreadPool;

pub fn initialise_connection(pool_size: usize) {
    let args: Vec<String> = env::args().collect();
    let port: &String = &args[1];
    let loopback: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket: SocketAddrV4 = SocketAddrV4::new(loopback, port.parse().unwrap());
    let listener: TcpListener = TcpListener::bind(socket).unwrap();

    println!("Rustic Server is listening on port {}", port);

    handle_connection(listener, pool_size);
}

fn handle_connection(listener: TcpListener, pool_size: usize){
    let pool: ThreadPool = rs_concurrency::ThreadPool::new(pool_size);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client: {:?}", stream);

                pool.execute( || {
                    request::handle_request(stream);
                });
            }
            Err(e) => println!("Unable to get the new client: {:?}", e),
        }
    }
}
