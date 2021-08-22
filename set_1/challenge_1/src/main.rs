use hex;
use base64;

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let decoded_hex = hex::decode(hex).unwrap();
    let actual_base64 = base64::encode(&decoded_hex);
    if expected_base64 == actual_base64 {
        println!("conversion succeeded! {} == {}", expected_base64, actual_base64);
    }
    else {
        println!("conversion failed! {} != {}", expected_base64, actual_base64);
    }
}
