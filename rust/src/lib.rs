mod binary_converter;
mod cipher;
mod rng;

pub use binary_converter::BytesBitsConverter;
pub use cipher::XorCipher;
pub use rng::{system_random, LinearCongruentialGenerator};

pub struct CryptConverter {
    xor_cipher: XorCipher,
    bytes_bits_converter: BytesBitsConverter,
}

impl CryptConverter {
    pub fn new(pwd: &[u8]) -> Self {
        Self {
            xor_cipher: XorCipher::new(pwd),
            bytes_bits_converter: BytesBitsConverter::new(),
        }
    }

    pub fn encode(&self, bytes: &[u8]) -> Vec<u8> {
        self.bytes_bits_converter
            .bytes_to_bits(&self.xor_cipher.encode(bytes))
    }

    pub fn decode(&self, bits: &[u8]) -> Vec<u8> {
        self.xor_cipher
            .decode(&self.bytes_bits_converter.bits_to_bytes(bits))
    }
}
