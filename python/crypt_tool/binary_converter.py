from typing import List


class BytesBitsConverter:
    def __init__(self):
        # 生成 byte 到 bit 的映射表
        self.byte_to_bits_map = [[(byte >> (7 - idx)) & 1 for idx in range(8)] for byte in range(256)]

    def bytes_to_bits(self, bytes1: bytes) -> List[int]:
        return [bit for byte in bytes1 for bit in self.byte_to_bits_map[byte]]

    def bits_to_bytes(self, bin1: List[int]) -> bytes:
        return bytes([
            sum(bit << (7 - i) for i, bit in enumerate(chunk))
            for chunk in (bin1[i:i + 8] for i in range(0, len(bin1), 8))
        ])
