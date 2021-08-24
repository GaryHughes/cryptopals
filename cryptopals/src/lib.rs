use maplit::hashmap;

pub fn score_english_text(input: &Vec<u8>) -> f64 {
    // From https://en.wikipedia.org/wiki/Letter_frequency
    // with the exception of ' ', which I estimated.

    let character_frequencies = hashmap!{
        'a' => 0.08167,
        'b' => 0.01492,
        'c' => 0.02782, 
        'd' => 0.04253,
        'e' => 0.12702,
        'f' => 0.02228,
        'g' => 0.02015, 
        'h' => 0.06094,
        'i' => 0.06094,
        'j' => 0.00153,
        'k' => 0.00772,
        'l' => 0.04025,
        'm' => 0.02406,
        'n' => 0.06749,
        'o' => 0.07507, 
        'p' => 0.01929,
        'q' => 0.00095, 
        'r' => 0.05987, 
        's' => 0.06327, 
        't' => 0.09056,
        'u' => 0.02758, 
        'v' => 0.00978, 
        'w' => 0.02360, 
        'x' => 0.00150,
        'y' => 0.01974,
        'z' => 0.00074, 
        ' ' => 0.13000
    };
  
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

pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter().zip(s2)
        .map(|(c1, c2)| c1 ^ c2)
        .map(|c| c.count_ones())
        .fold(0, |acc, x| acc + x)
}