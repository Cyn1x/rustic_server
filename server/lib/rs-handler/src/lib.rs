use rs_game;
use rs_game::Hangman;

pub struct Handler {
    game: rs_game::Hangman,
    response: Vec<u8>
}

impl Handler {
    pub fn new() -> Handler {
        let game: Hangman = Hangman::new();
        let response: Vec<u8> = Vec::new();

        Handler {
            game,
            response
        }
    }

    fn request_empty(&self, buffer: &Vec<u8>) -> bool {
        buffer.len() == 0 || buffer.starts_with(b"\r") || buffer.starts_with(b"\n")
    }

    fn request_invalid(&self, split_buffer: &Vec<u8>) -> bool {
        for &b in split_buffer {
            if !((b >= 65 && b <= 90) || (b >= 97 && b <= 122)) {
                return true
            }
        }
        false
    }

    fn new_game(&mut self) -> &[u8] {
        let response: Vec<u8> = self.game.get_hint().clone();
        return self.handle_response(response)
    }

    pub fn handle_request<'a>(&'a mut self, buffer: &'a Vec<u8>) -> &'a [u8] {
        let start_msg = b"START GAME";
        let invalid_msg = b"Invalid input detected\r\n";

        if self.request_empty(buffer) { return invalid_msg }

        if buffer.starts_with(start_msg) { return self.new_game() }

        let split_buffer: Vec<u8> = buffer.split(|&b | b == 13 || b == 10).next().unwrap().to_vec();

        if self.request_invalid(&split_buffer) { return invalid_msg }

        let response = self.game.verify_guess(&split_buffer).clone();
        self.handle_response(response)
    }

    fn handle_response(&mut self, response: Vec<u8>) -> &Vec<u8> {
        self.response = response;
        self.append_crlf();
        &self.response
    }

    fn append_crlf(&mut self) {
        self.response.push(13);
        self.response.push(10);
    }
}
