use base64;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let a = "this is a test".as_bytes();
    let b = "wokka wokka!!!".as_bytes();
    assert_eq!(hamming_distance(&a, &b), 37);  
    
    let data = read_file("6.txt");
    let keysize = get_key_size(&data);

    println!("keysize = {}", keysize);
}

fn get_key_size(data: &[u8]) -> i32 {
    for key_size in 2..=40 {
        let chunk_one = &data[0..key_size];
        let chunk_two = &data[key_size..];
        let distance = hamming_distance(&chunk_one, &chunk_two);
        let normalised_distance = distance / key_size as u32;
        println!("key_size = {}, distance = {}, normalised  {}", key_size, distance, normalised_distance);
    }
    0
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

fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter().zip(s2)
        .map(|(c1, c2)| c1 ^ c2)
        .map(|c| c.count_ones())
        .fold(0, |acc, x| acc + x)
}