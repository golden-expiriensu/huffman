use tree::Node;

mod encoding;
mod priority_queue;
mod tree;

pub fn encode(text: &str) -> Vec<u8> {
    Node::from(text).encode(text)
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
}
