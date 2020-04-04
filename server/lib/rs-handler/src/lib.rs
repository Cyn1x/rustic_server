use rs_game;
use rs_game::Hangman;

pub struct Handler {
    game: rs_game::Hangman

}

impl Handler {
    pub fn new() -> Handler {
        let game: Hangman = Hangman::new();

        Handler {
            game
        }
    }

    pub fn handle_request<'a>(&'a self, buffer: &'a Vec<u8>) -> &'a [u8] {
        let start_msg = b"START GAME";
        let invalid_msg = b"Invalid input detected";

        if buffer.starts_with(start_msg) {
            return self.game.get_hint();
        };

        for &x in buffer {
            if !(((x > 64 && x < 91) || (x > 97 && x < 123)) || (x == 10 || x == 13)) {
                return invalid_msg
            }
        }

        buffer
    }
}
