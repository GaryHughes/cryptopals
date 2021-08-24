pub mod pkcs7 {

pub fn pad(data: &mut Vec<u8>, block_size: usize) {
    let len = data.len();
    let padding = len % block_size;
    if padding > 0 {
        data.resize(data.len() + (block_size - padding), 0x04);
    }
}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_data_len_is_not_a_multiple_of_block_size() {
        let mut source = "YELLOW SUBMARINE".as_bytes().to_vec();
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec();
        pkcs7::pad(&mut source, 20);
        assert_eq!(source, expected);
    }

    #[test]
    fn pad_data_len_is_amultiple_of_block_size() {
        let mut source = "YELLOW".as_bytes().to_vec();
        let expected = "YELLOW".as_bytes().to_vec();
        pkcs7::pad(&mut source, 6);
        assert_eq!(source, expected);
    }

}
