use base64;
use std::collections::HashMap;
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strided::Stride;

fn main() {

    let a = "this is a test".as_bytes();
    let b = "wokka wokka!!!".as_bytes();
    assert_eq!(hamming_distance(&a, &b), 37);
    
    let data = read_file("6.txt");
    let key_size = get_key_size(&data);
    println!("keysize = {}", key_size);
    let key = get_key(&data, key_size);
    println!("{}", key);
    assert_eq!(key, "Terminator X: Bring the noise");


    // let file = File::open("6.txt").unwrap();
    // let reader = BufReader::new(file);
    // // TODO - change this to just keep the best candidate
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let mut key = key.as_bytes().iter().cycle();
    //     let decoded_bytes: Vec<u8> = line.chars().map(|c| c as u8).map(|b| b ^ key.next().unwrap()).collect();
    //     let decoded = str::from_utf8(&decoded_bytes).unwrap();
    //     println!("{}", decoded);
    // }
}


fn get_key(data: &[u8], key_size: usize) -> String {
    let all = Stride::new(&data);

    let strides = all.substrides(key_size);

    let mut key: Vec<u8> = Vec::new();

    for stride in strides {
        let bytes: Vec<&u8> = stride.iter().collect();
        let mut decoded: Vec<(u8, Vec<u8>, f64)> = Vec::new();
        for i in 0..256 {
            let key = i as u8;
            let decoded_bytes: Vec<u8> = bytes.iter().map(|x| *x ^ key).collect();
            let score = score_bytes(&decoded_bytes); 
            decoded.push((key, decoded_bytes, score));
        }
        decoded.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        match decoded.first() {
            Some(result) => key.push(result.0),
            None => println!("No result found")
        }
    }

    String::from_utf8(key).unwrap()
}

fn score_bytes(input: &Vec<u8>) -> f64 {
   
    // From https://en.wikipedia.org/wiki/Letter_frequency
    // with the exception of ' ', which I estimated.
    let mut character_frequencies: HashMap<char, f64> = HashMap::new();

    character_frequencies.insert('a', 0.08167); 
    character_frequencies.insert('b', 0.01492); 
    character_frequencies.insert('c', 0.02782); 
    character_frequencies.insert('d', 0.04253);
    character_frequencies.insert('e', 0.12702);
    character_frequencies.insert('f', 0.02228);
    character_frequencies.insert('g', 0.02015); 
    character_frequencies.insert('h', 0.06094);
    character_frequencies.insert('i', 0.06094);
    character_frequencies.insert('j', 0.00153);
    character_frequencies.insert('k', 0.00772);
    character_frequencies.insert('l', 0.04025);
    character_frequencies.insert('m', 0.02406);
    character_frequencies.insert('n', 0.06749);
    character_frequencies.insert('o', 0.07507); 
    character_frequencies.insert('p', 0.01929);
    character_frequencies.insert('q', 0.00095); 
    character_frequencies.insert('r', 0.05987); 
    character_frequencies.insert('s', 0.06327); 
    character_frequencies.insert('t', 0.09056);
    character_frequencies.insert('u', 0.02758); 
    character_frequencies.insert('v', 0.00978); 
    character_frequencies.insert('w', 0.02360); 
    character_frequencies.insert('x', 0.00150);
    character_frequencies.insert('y', 0.01974);
    character_frequencies.insert('z', 0.00074); 
    character_frequencies.insert(' ', 0.13000);
  
    input.iter()
        .map(|b| *b as char)
        .map(|c| {
            let lower = c.to_lowercase().collect::<Vec<_>>()[0];
            match character_frequencies.get(&lower) {
                Some(f) => *f,
                None => 0.0
            }
        })
        .sum()
}

fn get_key_size(data: &[u8]) -> usize {
    let mut result = 0usize;
    let mut smallest_avg = 0f64;
    for key_size in 2..=40 {
        let all: Vec<Vec<u8>> = data.chunks(key_size).map(|chunk| chunk.to_vec()).collect();
        let distances: Vec<f64> = all.chunks_exact(2)
            .map(|chunk| chunk.to_vec())
            .map(|chunk| {
                let first = &chunk[0];
                let second = &chunk[1];
                let distance = hamming_distance(&first, &second) as f64;
                let normalised_distance = distance / key_size as f64;
            normalised_distance as f64
            })
            .collect();

            let sum: f64 = distances.iter().sum();
            let count = distances.len() as f64;
            let avg = sum / count;

            if smallest_avg == 0.0 || avg < smallest_avg {
                smallest_avg = avg;
                result = key_size;
            }
        }

    result
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