use hex;
use std::str;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex::decode(input).unwrap();

    for key in b'A' ..= b'z' {
        let result_bytes: Vec<u8> = bytes.iter().map(|x| x ^ key).collect();
        let result = str::from_utf8(&result_bytes).unwrap();
        println!("{} {}", char::from(key), result);
    }
 

}

