use hex;

fn main() {
    let input = "1c0111001f010100061a024b53535009181c";
    let modifier = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";
    let decoded_bytes = hex::decode(input).unwrap();
    let modifer = hex::decode(modifier).unwrap();
    let actual: Vec<u8> = decoded_bytes.iter().zip(modifer.iter()).map(|(a, b)| -> u8 { a ^ b } ).collect();    
    let actual = hex::encode(actual);
    if expected == actual {
        println!("conversion succeeded! {} == {}", expected, actual);
    }
    else {
        println!("conversion failed! {} != {}", expected, actual);
    }
}
