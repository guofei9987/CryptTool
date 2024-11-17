from . import LCG
import copy


class XorCipher:
    def __init__(self, pwd: bytes):
        self.rng = LCG.from_seed(pwd)

    def apply_xor(self, data: bytes) -> bytes:
        rng = copy.copy(self.rng)
        return bytes([byte ^ rng.generate_u8() for byte in data])

    def encode(self, data: bytes) -> bytes:
        return self.apply_xor(data)

    def decode(self, data: bytes) -> bytes:
        return self.apply_xor(data)
