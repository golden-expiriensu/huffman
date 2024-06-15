use decoding::decode_text;
use encoding::text::encode_text;
use tree::Node;

mod decoding;
mod encoding;
mod priority_queue;
mod tree;

pub fn encode(text: &str) -> Vec<u8> {
    encode_text(&text.into(), text)
}

pub fn decode(tree: &Node, data: &[u8], bit_count: usize) -> String {
    decode_text(tree, data, bit_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let text = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";
        let expected = vec![
            0b10000111, 0b01001000, 0b11001001, 0b11011001, 0b11001001, 0b00011111, 0b00100111,
            0b11011111, 0b10001000, 0b11111101, 0b00111001, 0b00101111, 0b10111010, 0b00111111,
            0b00100000,
        ];
        assert_eq!(encode(text), expected);
    }

    #[test]
    fn test_decode() {
        let data = vec![
            0b10000111, 0b01001000, 0b11001001, 0b11011001, 0b11001001, 0b00011111, 0b00100111,
            0b11011111, 0b10001000, 0b11111101, 0b00111001, 0b00101111, 0b10111010, 0b00111111,
            0b00100000,
        ];
        let expected = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";
        assert_eq!(decode(&expected.into(), &data, 115), expected);
    }
}
