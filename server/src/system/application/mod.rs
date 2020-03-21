mod app {
    use crate::system;
    use system::network;

    pub fn run() { network::create_connection(); }
}

pub fn create_application() { app::run(); }
