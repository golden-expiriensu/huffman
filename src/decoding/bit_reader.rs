pub(super) struct BitReader<'a> {
    src: &'a [u8],
    limit: usize,
    at_byte: usize,
    at_bit: u8,
}

impl<'a> BitReader<'a> {
    pub(super) fn new(src: &'a [u8], limit: usize) -> Self {
        BitReader {
            src,
            limit,
            at_byte: 0,
            at_bit: 128, // Leftmost bit
        }
    }

    pub(super) fn next(&mut self) -> Option<bool> {
        if self.limit == 0 {
            return None;
        }
        self.limit -= 1;

        let byte = self.src.get(self.at_byte);

        match byte {
            Some(byte) => {
                let bit = byte & self.at_bit;

                if self.at_bit == 1 {
                    // Go get next byte
                    self.at_bit = 128;
                    self.at_byte += 1;
                } else {
                    // Go get next bit
                    self.at_bit /= 2;
                }

                Some(bit != 0)
            }
            None => None,
        }
    }
}
