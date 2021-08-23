use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("8.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut counts: HashMap<String, u32> = HashMap::new();
            line.chars()
                .collect::<Vec<char>>()
                .chunks(16)
                .map(|c| c.iter().collect::<String>())
                .for_each(|chunk| {
                    match counts.get(&chunk) {
                        Some(count) => counts.insert(chunk, count + 1),
                        None => counts.insert(chunk, 0)
                    };
                });

            (line, counts.values().sum::<u32>())
        })
        .for_each(|(line, count)| {
            if count > 0 {
                println!("{}: {}", line, count)
            }
        });
}

