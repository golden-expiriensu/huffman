# Huffman coding

## Description
This library does not encode the tree itself. Clients should use other serialization techniques to save or transmit the tree.

## Algorithm
https://en.wikipedia.org/wiki/Huffman_coding

## API

### Overview

```rust
use huffman::{encode, decode};
                                                             
let text = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";
                                                             
let (tree, blob, bit_count) = encode(text);
                                                             
assert_eq!(decode(&tree, &blob, bit_count), text);
```

### Encoding

```rust
use huffman::encode;

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
```

### Decoding

```rust
use huffman::decode;

let data = vec![
    0b10000111, 0b01001000, 0b11001001, 0b11011001, 0b11001001, 0b00011111, 0b00100111,
    0b11011111, 0b10001000, 0b11111101, 0b00111001, 0b00101111, 0b10111010, 0b00111111,
    0b00100000,
];
let size = 115;

let expected = "A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED";

assert_eq!(decode(&expected.into(), &data, size), expected);
```
