pub struct Hangman {
    server_word: Vec<char>,
    client_word: Vec<char>,
}

impl Hangman {
    pub fn new() -> Hangman {
        let secret_word: (Vec<char>, Vec<char>) = self::Hangman::create_word();
        let server_word: Vec<char> = secret_word.0;
        let client_word: Vec<char> = secret_word.1;

        Hangman {
            server_word,
            client_word
        }
    }

    fn create_word() -> (Vec<char>, Vec<char>) {
        let mut server_word: Vec<char> = Vec::new();
        let mut client_word: Vec<char> = Vec::new();
        let word: &str = "Apple";

        for char in word.chars() {
            server_word.push(char);
            client_word.push('_');
        }

        (server_word, client_word)
    }

    pub fn get_word(self) -> String {
        let mut server_word: String = String::new();

        for char in self.server_word {
            server_word.push(char);
        }

        server_word
    }

    pub fn verify_guess(&self, guess: &str) -> bool {
        true
    }

    pub fn check_guess(&self) -> &str {
        ""
    }
}
