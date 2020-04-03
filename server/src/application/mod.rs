pub mod handler;

use crate::system;
use crate::application::handler::Messenger;

pub fn run() {
    println!("Rustic Server initialised");

    let messenger: Messenger = handler::Messenger::new();

    system::network::initialise_connection();
}
