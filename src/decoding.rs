use crate::tree::Node;

use self::{bit_reader::BitReader, symbol::decode_symbol};

mod bit_reader;
mod symbol;

pub(crate) fn decode_text(tree: &Node, data: &[u8], bit_count: usize) -> String {
    let mut reader = BitReader::new(data, bit_count);
    let mut text = String::new();

    while let Some(symbol) = decode_symbol(tree, &mut reader) {
        text.push(symbol);
    }

    text
}
