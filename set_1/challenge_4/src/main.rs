use hex;
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use cryptopals::score_english_text;

fn main() {
    let file = File::open("4.txt").unwrap();
    let reader = BufReader::new(file);
    // TODO - change this to just keep the best candidate
    let mut decoded: Vec<(char, Vec<u8>, f64)> = Vec::new();
    for line in reader.lines() {
        let hex_line = line.unwrap();
        let bytes = hex::decode(hex_line).unwrap();
        for key in b'0' ..= b'z' {
            let decoded_bytes: Vec<u8> = bytes.iter().map(|x| x ^ key).collect();
            let score = score_english_text(&decoded_bytes); 
            decoded.push((char::from(key), decoded_bytes, score));
        }
    }
    decoded.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    match decoded.first() {
        Some(result) => println!("{} {}", result.0, str::from_utf8(&result.1).unwrap()),
        None => println!("No result found")
    }
}
