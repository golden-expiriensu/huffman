use crate::tree::Node;

use super::symbol::Symbol;

pub(crate) fn encode_text(tree: &Node, text: &str) -> (Vec<u8>, usize) {
    let mut blob = Vec::new();
    let mut acum = Symbol::new();

    for symbol in text.chars() {
        if let Some(overflow) = acum.absorb(Symbol::encode(tree, symbol)) {
            blob.push(overflow);
        }
    }

    let mut bit_count = blob.len() * 8;

    if let Some((byte, number_of_bits)) = acum.destruct() {
        blob.push(byte);
        bit_count += number_of_bits;
    }

    (blob, bit_count)
}
