mod app {
    use crate::system;

    pub fn run() { system::network::create_connection(); }
}

pub fn create_application() { app::run(); }
