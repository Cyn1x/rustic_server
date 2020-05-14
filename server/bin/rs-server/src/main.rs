//! # Rustic Server
//!
//! `rs-server` is the entry point to the program. It initialises the `rs-network` crate.

use rs_network;

/// Initialises the network to accept incoming connections.
fn main() {
    println!("[Server]: Rustic Server initialised");

    rs_network::initialise_connection();
}
