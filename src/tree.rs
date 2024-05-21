use std::collections::BTreeMap;

use crate::priority_queue::Queue;

// In Huffman coding node with lowest probability is given highest priority.
const INVERSE_WEIGHT: usize = usize::MAX;

#[derive(Debug, PartialEq)]
pub(crate) enum Node {
    Branch {
        tip: Vec<char>,
        left: Box<Node>,
        right: Box<Node>,
    },
    Leaf(char),
}

impl Node {
    pub(crate) fn contains(&self, symbol: char) -> bool {
        match self {
            Node::Branch {
                tip,
                left: _,
                right: _,
            } => tip.contains(&symbol),
            Node::Leaf(value) => symbol == *value,
        }
    }

    fn cloned(&self) -> Vec<char> {
        match self {
            Node::Branch {
                tip,
                left: _,
                right: _,
            } => tip.clone(),
            Node::Leaf(char) => vec![char.clone()],
        }
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut index: BTreeMap<char, usize> = BTreeMap::new();
        let mut order: Vec<char> = Vec::new();

        for symbol in value.chars() {
            index
                .entry(symbol)
                .and_modify(|count| *count += 1)
                .or_insert_with(|| {
                    order.push(symbol);
                    1
                });
        }

        let mut queue = Queue::new();
        for symbol in order {
            queue.enqueue(Node::Leaf(symbol), INVERSE_WEIGHT - index[&symbol]);
        }

        queue.into()
    }
}

impl From<Queue<Node>> for Node {
    fn from(mut value: Queue<Node>) -> Self {
        while let Some(left) = value.dequeue() {
            match value.dequeue() {
                Some(right) => value.enqueue(
                    Node::Branch {
                        tip: [left.value.cloned(), right.value.cloned()].concat(),
                        left: Box::new(left.value),
                        right: Box::new(right.value),
                    },
                    INVERSE_WEIGHT
                        - (INVERSE_WEIGHT - left.weight)
                        - (INVERSE_WEIGHT - right.weight),
                ),
                None => return left.value,
            }
        }
        Node::Leaf('\0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let text = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";
        assert_eq!(
            Node::from(text),
            Node::Branch {
                tip: vec![' ', 'D', 'A', 'E', 'C', 'B'],
                left: Box::new(Node::Branch {
                    tip: vec![' ', 'D'],
                    left: Box::new(Node::Leaf(' ')),
                    right: Box::new(Node::Leaf('D')),
                }),
                right: Box::new(Node::Branch {
                    tip: vec!['A', 'E', 'C', 'B'],
                    left: Box::new(Node::Leaf('A')),
                    right: Box::new(Node::Branch {
                        tip: vec!['E', 'C', 'B'],
                        left: Box::new(Node::Leaf('E')),
                        right: Box::new(Node::Branch {
                            tip: vec!['C', 'B'],
                            left: Box::new(Node::Leaf('C')),
                            right: Box::new(Node::Leaf('B')),
                        }),
                    }),
                }),
            },
        );
    }
}
