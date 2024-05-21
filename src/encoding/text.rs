use crate::tree::Node;

use super::symbol::Symbol;

pub(crate) fn encode_text(tree: &Node, text: &str) -> Vec<u8> {
    let mut res = Vec::new();
    let mut acc = Symbol::new();

    for symbol in text.chars() {
        if let Some(overflow) = acc.absorb(Symbol::encode(tree, symbol)) {
            res.push(overflow);
        }
    }

    if let Some(incomplete) = acc.destruct() {
        res.push(incomplete);
    }
    res
}
