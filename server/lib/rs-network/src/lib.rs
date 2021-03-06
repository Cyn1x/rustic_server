//! Rustic Server Network
//!
//! `rs-network` initialises the server and listens for incoming requests,
//! then processes them.

mod request;
mod response;

use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype, SslStream, SslAcceptorBuilder};

use rs_concurrency;
use rs_concurrency::ThreadPool;

/// Initialises the server listener and calls another function to handle the listener.
pub fn initialise_connection() {
    let args: Vec<String> = env::args().collect();
    let port: &String = &args[1];
    let loopback: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let socket: SocketAddrV4 = SocketAddrV4::new(loopback, port.parse()
        .expect("Error creating socket"));
    let listener: TcpListener = TcpListener::bind(socket)
        .expect("Error creating TCP listener");

    let mut acceptor: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("cert.pem").unwrap();
    acceptor.check_private_key().unwrap();

    let acceptor: Arc<SslAcceptor> = Arc::new(acceptor.build());

    println!("[Server]: Rustic Server is listening on port {}", port);

    handle_connection(&listener, &acceptor);
}

/// Listens for incoming requests then dispatches them for processing. The `rs-concurrency` crate
/// concurrently handles multiple requests. The thread pool has a maximum limit of 100
/// concurrent requests at a time.
fn handle_connection(listener: &TcpListener, acceptor: &Arc<SslAcceptor>){
    let pool_size: usize = 100;
    let pool: ThreadPool = rs_concurrency::ThreadPool::new(pool_size);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor: Arc<SslAcceptor> = acceptor.clone();

                println!("[Server]: New client: {:?}", stream);

                pool.execute(move || {
                    handle_client(stream, &acceptor)
                });
            }
            Err(e) => println!("[Server]: Unable to get new client: {:?}", e),
        }
    }

    println!("[Server]: Rustic Server is shutting down.");
}

fn handle_client(stream: TcpStream, acceptor: &Arc<SslAcceptor>) {
    let timeout_duration: Duration = Duration::new(60, 0);

    let stream: SslStream<TcpStream> = acceptor.accept(stream).unwrap();

    stream.get_ref().set_read_timeout(Option::from(timeout_duration))
        .expect("Failed to set the read timout.");

    request::handle_request(stream);
}
