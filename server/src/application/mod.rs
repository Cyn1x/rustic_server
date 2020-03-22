mod app {
    use crate::system;
    use crate::game;

    use system::network;

    pub fn run() {
        game::create_game();
        network::create_connection();
    }
}

pub fn create_application() { app::run(); }
