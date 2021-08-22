use hex;
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("4.txt").unwrap();
    let reader = BufReader::new(file);
    // TODO - change this to just keep the best candidate
    let mut decoded: Vec<(char, Vec<u8>, u32)> = Vec::new();
    for line in reader.lines() {
        let hex_line = line.unwrap();
        let bytes = hex::decode(hex_line).unwrap();
        for key in b'0' ..= b'z' {
            let decoded_bytes: Vec<u8> = bytes.iter().map(|x| x ^ key).collect();
            let score = score_bytes(&decoded_bytes); 
            decoded.push((char::from(key), decoded_bytes, score));
        }
    }
    decoded.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    match decoded.first() {
        Some(result) => println!("{} {}", result.0, str::from_utf8(&result.1).unwrap()),
        None => println!("No result found")
    }
}

fn score_bytes(input: &Vec<u8>) -> u32 {
    input.iter()
        .map(|b| *b as char)
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() { 2 } 
            else if c.is_ascii_punctuation() { 1 }
            else { 0 }
        }).sum()
}
