use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

pub struct Hangman {
    server_word: Vec<u8>,
    client_word: Vec<u8>,
    game_state: State
}

struct State {
    char_guesses: i32,
    word_guesses: i32
}

impl Hangman {
    pub fn new() -> Hangman {
        let secret_word: (Vec<u8>, Vec<u8>) = self::Hangman::create_word();
        let server_word: Vec<u8> = secret_word.0;
        let client_word: Vec<u8> = secret_word.1;

        let (char_guesses, word_guesses) = (0, 0);

        let game_state = State {
            char_guesses,
            word_guesses
        };

        Hangman {
            server_word,
            client_word,
            game_state
        }
    }

    pub fn create_word() -> (Vec<u8>, Vec<u8>) {
        let mut file = File::open("server/var/words.txt").expect("File not found");
        let mut contents: String = String::new();

        file.read_to_string(&mut contents).unwrap();

        let secret_word: String = self::Hangman::choose_word(&contents);
        let server_word: Vec<u8> = secret_word.into_bytes();
        let client_word: Vec<u8> = vec![95; server_word.len()];

        (server_word, client_word)
    }

    pub fn choose_word(contents: &String) -> String {
        let lines = contents.lines().count();
        let mut secret_word: String = String::new();
        let random_number = rand::thread_rng().gen_range(0, lines);

        contents.lines().into_iter().enumerate().for_each( | word | {
            if word.0 == random_number {
                secret_word.push_str(word.1)
            }
        });

        secret_word
    }

    pub fn verify_guess(&mut self, buffer: &Vec<u8>) -> &Vec<u8> {
        if buffer.len() > 1 { self.handle_word(buffer) }
        else { self.handle_char(&buffer[0]); }

        if self.game_over() { return self.construct_summary() }

        &self.client_word
    }

    fn handle_char(&mut self, byte: &u8) {
        let server_word: &Vec<u8> = &self.server_word;
        let client_word: &mut Vec<u8> = &mut self.client_word;
        let char_guesses = &mut self.game_state.char_guesses;

        server_word.iter().enumerate().for_each( | (i, &b) | {
            if b.eq_ignore_ascii_case(byte) && client_word[i] == 95 {
                client_word[i] = b;
                *char_guesses += 1;
            }
        });
    }

    fn handle_word(&mut self, buffer: &Vec<u8>) {
        let client_word: &mut Vec<u8> = &mut self.client_word;

        if self.server_word.eq(buffer) { client_word.clone_from_slice(&buffer[..]); }

        self.game_state.word_guesses += 1;
    }

    fn game_over(&self) -> bool {
        self.server_word.eq(&self.client_word)
    }

    fn construct_summary(&mut self) -> &Vec<u8> {
        let score: i32 = self.calculate_score();
        let mut client_summary = String::from(format!("\r\n{} \r\nGAME OVER", score)).into_bytes();

        &self.client_word.append(client_summary.as_mut());

        &self.client_word
    }

    fn calculate_score(&self) -> i32 {
        10 * (self.client_word.len() as i32) - 2 * (self.game_state.char_guesses) - (self.game_state.word_guesses)
    }

    pub fn get_hint(&self) -> &Vec<u8> {
        &self.client_word
    }
}
