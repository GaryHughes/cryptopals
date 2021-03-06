use hex;

fn main() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let mut key = "ICE".as_bytes().iter().cycle();
    let actual_bytes: Vec<u8> = input.chars().map(|c| c as u8).map(|b| b ^ key.next().unwrap()).collect();
    let actual = hex::encode(actual_bytes);
    assert_eq!(expected, actual);
}
