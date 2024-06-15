use crate::tree::Node;

use super::bit_reader::BitReader;

pub(super) fn decode_symbol(tree: &Node, reader: &mut BitReader) -> Option<char> {
    match tree {
        Node::Branch {
            tip: _,
            left,
            right,
        } => {
            if let Some(bit) = reader.next() {
                if bit {
                    decode_symbol(right, reader)
                } else {
                    decode_symbol(left, reader)
                }
            } else {
                return None;
            }
        }
        Node::Leaf(value) => Some(value.clone()),
    }
}
