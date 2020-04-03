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

    pub fn handle_request(&self, guess: &str) -> &str {
        let valid: bool = self.game.verify_guess(guess);

        if valid {
            self.game.check_guess();
            "TEMP PLACEHOLDER: VALID"
        }
        else { "TEMP PLACEHOLDER: INVALID" }
    }
}
