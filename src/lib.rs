use decoding::decode_text;
use encoding::encode_text;
use tree::Node;

mod decoding;
mod encoding;
mod priority_queue;
mod tree;

/// Returns the Huffman tree of the text, the encoded data blob as a byte array, and the size of
/// the blob in bits.
///
/// The binary size is necessary for decoding the last byte of the blob because there's no
/// definitive way to determine if trailing zeros are valid encoded symbols or simply padding for
/// the byte.
///
/// The encoded blob does not include the tree itself. Therefore, the calling client must serialize
/// the tree separately if they intend to save or transmit it as binary data.
///
/// # Examples
///
/// ```
/// use huffman::{encode, decode};
///
/// let text = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";
///
/// let (tree, blob, bit_count) = encode(text);
///
/// assert_eq!(decode(&tree, &blob, bit_count), text);
/// ```
pub fn encode(text: &str) -> (Node, Vec<u8>, usize) {
    let tree = text.into();
    let (blob, bit_count) = encode_text(&tree, text);
    (tree, blob, bit_count)
}

/// Decodes the Huffman-encoded data using the provided Huffman tree and returns the decoded text.
///
/// The actual number of bits in the encoded data blob is required to properly decode the last byte
/// of the blob, as it may contain padding.
///
/// # Examples
///
/// ```
/// use huffman::{encode, decode};
///
/// let text = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";
///
/// let (tree, blob, bit_count) = encode(text);
///
/// assert_eq!(decode(&tree, &blob, bit_count), text);
/// ```
pub fn decode(tree: &Node, data: &[u8], bit_count: usize) -> String {
    decode_text(tree, data, bit_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let text = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";

        let expected_blob = vec![
            0b10000111, 0b01001000, 0b11001001, 0b11011001, 0b11001001, 0b00011111, 0b00100111,
            0b11011111, 0b10001000, 0b11111101, 0b00111001, 0b00101111, 0b10111010, 0b00111111,
            0b00100000,
        ];
        let expected_size = 115;

        let (_, blob, bit_count) = encode(text);

        assert_eq!(expected_blob, blob);
        assert_eq!(expected_size, bit_count);
    }

    #[test]
    fn test_decode() {
        let data = vec![
            0b10000111, 0b01001000, 0b11001001, 0b11011001, 0b11001001, 0b00011111, 0b00100111,
            0b11011111, 0b10001000, 0b11111101, 0b00111001, 0b00101111, 0b10111010, 0b00111111,
            0b00100000,
        ];
        let size = 115;

        let expected = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";

        assert_eq!(decode(&expected.into(), &data, size), expected);
    }
}
