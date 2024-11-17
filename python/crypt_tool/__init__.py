from .rng import LCG, system_random
from .cipher import XorCipher
from .binary_converter import BytesBitsConverter
from typing import List


class CryptConverter:
    def __init__(self, pwd: bytes):
        self.xor_cipher = XorCipher(pwd)
        self.bytes_bits_converter = BytesBitsConverter()

    def encode(self, data: bytes):
        return self.bytes_bits_converter.bytes_to_bits(self.xor_cipher.encode(data))

    def decode(self, bin1: List[int]):
        return self.xor_cipher.decode(self.bytes_bits_converter.bits_to_bytes(bin1=bin1))
