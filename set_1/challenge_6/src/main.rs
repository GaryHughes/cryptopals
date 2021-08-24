use base64;
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strided::Stride;
use cryptopals::{score_english_text, hamming_distance};

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

    let file = File::open("6.txt").unwrap();
    let reader = BufReader::new(file);
    let mut key = key.as_bytes().iter().cycle();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = base64::decode(line).unwrap();
        let decoded_bytes: Vec<u8> = line.iter().map(|b| b ^ key.next().unwrap()).collect();
        let decoded = str::from_utf8(&decoded_bytes).unwrap();
        print!("{}", decoded);
    }
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
            let score = score_english_text(&decoded_bytes); 
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

