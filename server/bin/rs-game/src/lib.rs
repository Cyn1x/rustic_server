use std::time::SystemTime;

pub struct Hangman {
    server_word: Vec<u8>,
    client_word: Vec<u8>
}

impl Hangman {
    pub fn new() -> Hangman {
        let secret_word: (Vec<u8>, Vec<u8>) = self::Hangman::create_word();
        let server_word: Vec<u8> = secret_word.0;
        let client_word: Vec<u8> = secret_word.1;

        Hangman {
            server_word,
            client_word
        }
    }

    fn create_word() -> (Vec<u8>, Vec<u8>) {
        let secret_word: String = String::from("apple");
        let server_word: Vec<u8> = secret_word.into_bytes();
        let client_word: Vec<u8> = vec![95; server_word.len()];

        (server_word, client_word)
    }

    fn update_word() -> &Vec<u8> {

    }

    pub fn check_guess(&self) {

    }

    pub fn get_hint(&self) -> &Vec<u8> {
        &self.client_word
    }
}
