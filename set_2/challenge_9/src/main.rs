use cryptopals::pkcs7;

fn main() {
    let mut source = "YELLOW SUBMARINE".as_bytes().to_vec();
    let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec();
    pkcs7::pad(&mut source, 20);
    assert_eq!(source, expected);
}
