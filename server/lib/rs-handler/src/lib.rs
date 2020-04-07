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

    pub fn handle_request<'a>(&'a mut self, buffer: &'a Vec<u8>) -> &'a [u8] {
        let start_msg = b"START GAME";
        let invalid_msg = b"Invalid input detected";
        let mut split_buffer: Vec<u8> = buffer.split(|&b | b == 10).next().unwrap().to_vec();

        if split_buffer.starts_with(start_msg) {
            return self.game.get_hint()
        };

        for &b in &split_buffer {
            if !((b >= 64 && b <= 91) || (b >= 97 && b <= 123)) {
                return invalid_msg
            }
        }

        self.game.verify_guess(&mut split_buffer)
    }
}
