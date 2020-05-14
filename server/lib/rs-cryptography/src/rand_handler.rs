use openssl::rand::rand_bytes;

pub fn generate_sequence(size: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; size];
    rand_bytes(&mut buffer).unwrap();

    buffer
}
