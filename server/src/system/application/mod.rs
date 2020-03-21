mod app {
    use crate::system;
    use system::network;
    use crate::game;

    pub fn run() {
        game::create_game();
        network::create_connection();
    }
}

pub fn create_application() { app::run(); }
