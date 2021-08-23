use openssl::symm::{decrypt, Cipher};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use base64;

fn main() {
    let key = "YELLOW SUBMARINE".as_bytes();
    let cipher = Cipher::aes_128_ecb();
    let data = read_file("7.txt");
    let plaintext = decrypt(cipher, key, None, &data).unwrap();
    let plaintext = str::from_utf8(&plaintext).unwrap();
    println!("{}", plaintext);
}

fn read_file(filename: &str) -> Vec<u8> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // TODO - can this be done more better?
    let mut buffer: Vec<u8> = Vec::new();
    reader.lines()
        .map(|line| base64::decode(line.unwrap()).unwrap())
        .for_each(|chunk| {
            let mut chunk_buf = chunk.clone();
            buffer.append(&mut chunk_buf);
        });
    buffer
}
