//! Rustic Server Hangman
//!
//! `rs-hangman` initialises Hangman game and processes the state at each
//! stage of the game.

use std::fs::File;
use std::io::prelude::*;

use rand::Rng;

use rs_cryptography;
use rs_cryptography::aes_handler::AES;

/// The game data
pub struct Hangman {
    server_word: Vec<u8>,
    client_word: Vec<u8>,
    game_state: State
}

/// The game state data
struct State {
    char_guesses: i32,
    word_guesses: i32
}

impl Hangman {
    /// Returns a game of Hangman with a secret word.
    ///
    /// # Arguments
    ///
    /// * `server_word` - An 8-bit unsigned integer vector that contains the secret word
    /// * `client_word` - An 8-bit unsigned inteneger vector that contains the hidden word
    /// * `game_state`  - Contains the gane state data
    ///
    /// # Example
    ///
    /// ```
    /// // Example structure instantiation.
    /// use rs_game::Hangman;
    /// let game = Hangman::new();
    /// ```
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

    /// Streams the data from the file that contains a list of words. This function sets up the
    /// file access and calls another function to choose which word to store. Returns the
    /// server word and client word vectors.
    pub fn create_word() -> (Vec<u8>, Vec<u8>) {
        let mut aes_handler: AES = rs_cryptography::aes_handler::AES::new();

        // Plaintext file remains here only for demonstration purposes
        aes_handler.encrypt_file("server/var/words.txt");

        let decrypted_words: String = aes_handler.decrypt_file(
            "server/var/encrypted_words.txt");

        let secret_word: String = self::Hangman::choose_word(&decrypted_words);
        let server_word: Vec<u8> = secret_word.into_bytes();
        let client_word: Vec<u8> = vec![95; server_word.len()];

        (server_word, client_word)
    }

    /// Uses a random generator to choose a random word in the file. Returns the secret word.
    pub fn choose_word(contents: &String) -> String {
        let lines: usize = contents.lines().count();
        let mut secret_word: String = String::new();
        let random_number: usize = rand::thread_rng().gen_range(0, lines);

        contents.lines().into_iter().enumerate().for_each( | word | {
            if word.0 == random_number {
                secret_word.push_str(word.1)
            }
        });

        secret_word
    }

    /// Determins whether the guess is a char or word guess. The appropriate functions handle
    /// each case. Returns the client word.
    pub fn verify_guess(&mut self, buffer: &Vec<u8>) -> &Vec<u8> {
        if buffer.len() > 1 { self.handle_word(buffer) }
        else { self.handle_char(&buffer[0]); }

        if self.game_over() { return self.construct_summary() }

        &self.client_word
    }

    /// Checks whether the guessed byte appears in any index in the server word vector. If the
    /// guess is correct, the byte gets placed in the appropriate position in the client vector,
    /// overwriting the underscore.
    fn handle_char(&mut self, byte: &u8) {
        let server_word: &Vec<u8> = &self.server_word;
        let client_word: &mut Vec<u8> = &mut self.client_word;

        server_word.iter().enumerate().for_each( | (i, &b) | {
            if b.eq_ignore_ascii_case(byte) && client_word[i] == 95 {
                client_word[i] = b;
            }
        });

        self.game_state.char_guesses += 1;
    }

    /// Checks whether the guessed word equals the server word vector. The buffer transforms to
    /// ASCII lower case. If the guess is correct, the buffer gets cloned to the client vector.
    fn handle_word(&mut self, buffer: &Vec<u8>) {
        let buffer: Vec<u8> = buffer.iter()
            .map( | b | b.to_ascii_lowercase()).collect();

        if self.server_word.eq(&buffer) { self.client_word.clone_from_slice(&buffer[..]); }

        self.game_state.word_guesses += 1;
    }

    /// Checks whether the game is over by comparing the client and server word vectors.
    /// Returns true or false.
    fn game_over(&self) -> bool { self.server_word.eq(&self.client_word) }

    /// Constructs a summary for the client by concatenating a message with the score, and appends
    /// this to the client word vector. Returns the client word vector.
    fn construct_summary(&mut self) -> &Vec<u8> {
        let score: i32 = self.calculate_score();
        let mut client_summary = String::from(format!("\n{} \nGAME OVER\n", score))
            .into_bytes();

        &self.client_word.append(client_summary.as_mut());

        &self.client_word
    }

    /// Calculates the final score with the formula:
    ///  `10 * (secret word length) - 2 * (number of char guesses) - (number of word guesses)`
    fn calculate_score(&self) -> i32 {
        10 * (self.client_word.len() as i32) - 2 * (self.game_state.char_guesses)
            - (self.game_state.word_guesses)
    }

    /// Returns the hidden client word.
    pub fn get_hint(&self) -> &Vec<u8> { &self.client_word }

    /// Returns the revealed server word.
    pub fn get_word(&self) -> &Vec<u8> { &self.server_word }
}
