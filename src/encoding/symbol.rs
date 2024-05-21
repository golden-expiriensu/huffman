use crate::tree::Node;

pub(super) struct Symbol {
    body: u8,
    size: usize,
}

impl Symbol {
    pub(super) fn new() -> Self {
        Self { body: 0, size: 0 }
    }

    pub(super) fn destruct(self) -> Option<u8> {
        if self.size > 0 {
            Some(self.body)
        } else {
            None
        }
    }

    pub(super) fn absorb(&mut self, other: Self) -> Option<u8> {
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

    pub(super) fn encode(tree: &Node, symbol: char) -> Self {
        Symbol::new().encode_rec(tree, symbol)
    }

    fn encode_rec(mut self, tree: &Node, symbol: char) -> Self {
        match tree {
            Node::Branch {
                tip: _,
                left,
                right,
            } => {
                if left.contains(symbol) {
                    self.size += 1;
                    self.encode_rec(left, symbol)
                } else {
                    self.body |= 0b1000_0000 >> self.size;
                    self.size += 1;
                    self.encode_rec(right, symbol)
                }
            }
            Node::Leaf(_) => self,
        }
    }
}
