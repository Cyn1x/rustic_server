use crate::game;
use crate::game::Hangman;

pub struct Messenger {
    game: game::Hangman
}

impl Messenger {
    pub fn new() -> Messenger {
        let game: Hangman = game::Hangman::new();

        Messenger {
            game
        }
    }

    pub fn get(&self) -> &Hangman {
        &self.game
    }
}
