use crate::tree::Node;

pub(crate) struct Encoding {
    body: u8,
    size: usize,
}

impl Encoding {
    pub(crate) fn new() -> Self {
        Self { body: 0, size: 0 }
    }

    pub(crate) fn symbol(tree: &Node, symbol: char) -> Self {
        encode_symbol(Encoding::new(), tree, symbol)
    }

    pub(crate) fn absorb(&mut self, other: Self) -> Option<u8> {
        if self.size + other.size < 8 {
            self.body |= other.body >> self.size;
            self.size += other.size;
            return None;
        }
        let overflow = self.body | (other.body >> self.size);
        self.size = self.size + other.size - 8;
        self.body = other.body << (other.size - self.size);
        Some(overflow)
    }

    pub(crate) fn destruct(self) -> Option<u8> {
        if self.size > 0 {
            Some(self.body)
        } else {
            None
        }
    }
}

fn encode_symbol(mut binary: Encoding, node: &Node, symbol: char) -> Encoding {
    match node {
        Node::Branch {
            tip: _,
            left,
            right,
        } => {
            if left.contains(symbol) {
                binary.size += 1;
                encode_symbol(binary, left, symbol)
            } else {
                binary.body |= 0b1000_0000 >> binary.size;
                binary.size += 1;
                encode_symbol(binary, right, symbol)
            }
        }
        Node::Leaf(_) => binary,
    }
}
