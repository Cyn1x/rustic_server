mod request;
mod response;

use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::thread;
use std::io::Write;

use rs_concurrency;
use rs_concurrency::ThreadPool;

pub fn initialise_connection() {
    let args: Vec<String> = env::args().collect();
    let port: &String = &args[1];
    let loopback: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket: SocketAddrV4 = SocketAddrV4::new(loopback, port.parse().unwrap());
    let listener: TcpListener = TcpListener::bind(socket).unwrap();

    println!("Rustic Server is listening on port {}", port);

    handle_connection(listener);
}

fn handle_connection(listener: TcpListener){
    const POOL_SIZE: usize = 100;
    let pool: ThreadPool = rs_concurrency::ThreadPool::new(POOL_SIZE);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client: {:?}", stream);

                pool.execute( || {
                    request::handle_request(stream);
                });
            }
            Err(e) => println!("Unable to get the new client: {:?}", e),
        }
    }

}
