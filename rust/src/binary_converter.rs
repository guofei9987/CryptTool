use std::array::from_fn;

pub struct BytesBitsConverter {
    byte_to_bits_map: [[u8; 8]; 256],
}

impl Default for BytesBitsConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl BytesBitsConverter {
    pub fn new() -> Self {
        Self {
            byte_to_bits_map: from_fn(|byte| from_fn(|idx| ((byte as u8 >> (7 - idx)) & 1))),
        }
    }

    pub fn bytes_to_bits(&self, bytes1: &[u8]) -> Vec<u8> {
        bytes1
            .iter()
            .flat_map(|byte1| self.byte_to_bits_map[*byte1 as usize])
            .collect::<Vec<u8>>()
    }

    pub fn bits_to_bytes(&self, bin1: &[u8]) -> Vec<u8> {
        bin1.chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, &bit)| acc | (bit << (7 - i)))
            })
            .collect::<Vec<u8>>()
    }
}
