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
        let mut client_word: Vec<u8> = secret_word.1;
        self::Hangman::append_crlf(&mut client_word);

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
        let secret_word: String = String::from("apple");
        let server_word: Vec<u8> = secret_word.into_bytes();
        let client_word: Vec<u8> = vec![95; server_word.len()];

        (server_word, client_word)
    }

    pub fn verify_guess(&mut self, buffer: &Vec<u8>) -> &Vec<u8> {
        let mut word = buffer.split(|&b | b == 13).next().unwrap().to_vec();

        if word.len() > 1 { self.handle_word(&mut word) }
        else { self.handle_char(&word[0]); }

        if self.game_over() { return self.construct_summary() }

        &self.client_word
    }

    fn append_crlf(buffer: &mut Vec<u8>) {
        buffer.push(13);
        buffer.push(10);
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

    fn handle_word(&mut self, buffer: &mut Vec<u8>) {
        let client_word: &mut Vec<u8> = &mut self.client_word;

        if self.server_word.eq(buffer) { client_word[..self.server_word.len()].clone_from_slice(&buffer[..]); }

        self.game_state.word_guesses += 1;
    }

    fn game_over(&self) -> bool {
        self.server_word[..].eq(&self.client_word[..self.server_word.len()])
    }

    fn construct_summary(&mut self) -> &Vec<u8> {
        let score: i32 = self.calculate_score();
        let mut client_summary = String::from(format!("\r\n{} \r\nGAME OVER", score)).into_bytes();
        self::Hangman::append_crlf(&mut client_summary);

        &self.client_word.append(client_summary.as_mut());

        &self.client_word
    }

    fn calculate_score(&self) -> i32 {
        10 * (self.client_word[..self.server_word.len()].len() as i32) - 2 * (self.game_state.char_guesses) - (self.game_state.word_guesses)
    }

    pub fn get_hint(&self) -> &Vec<u8> {
        &self.client_word
    }
}
