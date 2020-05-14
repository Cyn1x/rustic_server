use std::fs::File;
use std::io::{Read, Write};

use rand::Rng;
use openssl::symm::{Cipher, encrypt_aead, decrypt_aead};

use crate::rand_handler;
use std::mem;

pub struct AES {
    cipher: Cipher,
    key: Vec<u8>,
    iv: Vec<u8>,
    aad: Vec<u8>,
    tag: Vec<u8>,
}

impl AES {
    pub fn new() -> AES {
        let cipher: Cipher = Cipher::aes_256_gcm();
        let key: Vec<u8>= rand_handler::generate_sequence(32);
        let iv: Vec<u8> = rand_handler::generate_sequence(64);
        let arbitrary_size: usize = rand::thread_rng().gen_range(0, 256);
        let aad: Vec<u8> = rand_handler::generate_sequence(arbitrary_size);
        let tag: Vec<u8> = rand_handler::generate_sequence(16);

        AES {
            cipher,
            key,
            iv,
            aad,
            tag
        }
    }

    pub fn encrypt_file(&mut self, filepath: &str) {
        let mut file: File = File::open(filepath).expect("File not found.");
        let mut contents: String = String::new();

        file.read_to_string(&mut contents)
            .expect("Error streaming file contents to String buffer.");

        let encrypted_bytes: Vec<u8> = self.encrypt_data(
            contents.as_bytes());

        let mut encrypted_file: File = File::create(
            "server/var/encrypted_words.txt").expect("Unable to create file.");

        encrypted_file.write_all(&encrypted_bytes);
    }

    pub fn decrypt_file(&self, filepath: &str) -> String {
        let mut decrypted_file: File = File::open(filepath).expect("File not found.");
        let mut file_contents: Vec<u8> = Vec::new();

        decrypted_file.read_to_end(&mut file_contents)
            .expect("Error streaming file contents to String buffer.");

        let decrypted_bytes: Vec<u8> = self.decrypt_data(&file_contents);

        String::from_utf8_lossy(&decrypted_bytes[..]).parse().unwrap()
    }

    fn encrypt_data(&mut self, buffer: &[u8]) -> Vec<u8> {
        let data: &[u8] = buffer;
        let ciphertext: Vec<u8> = encrypt_aead(
            self.cipher,
            &self.key,
            Some(&self.iv),
            &self.aad,
            data,
            &mut self.tag
        ).expect("Unable to encrypt ciphertext.");

        ciphertext
    }

    fn decrypt_data(&self, buffer: &[u8]) -> Vec<u8> {
        let data: &[u8] = buffer;
        let ciphertext: Vec<u8> = decrypt_aead(
            self.cipher,
            &self.key,
            Some(&self.iv),
            &self.aad,
            data,
            &self.tag
        ).expect("Unable to decrypt ciphertext.");

        ciphertext
    }

}
