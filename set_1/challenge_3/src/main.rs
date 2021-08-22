use hex;
use std::str;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex::decode(input).unwrap();
    let mut decoded: Vec<(char, Vec<u8>, u32)> = Vec::new();  
    for key in b'A' ..= b'z' {
        let decoded_bytes: Vec<u8> = bytes.iter().map(|x| x ^ key).collect();
        let score = score_bytes(&decoded_bytes); 
        decoded.push((char::from(key), decoded_bytes, score));
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

