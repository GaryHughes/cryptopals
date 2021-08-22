
fn main() {
    assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);  
}

fn hamming_distance(s1: &str, s2: &str) -> u32 {
    s1.bytes().zip(s2.bytes()).map(|(c1, c2)| c1 ^ c2).map(|c| c.count_ones()).fold(0, |acc, x| acc + x)
}