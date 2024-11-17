import time


class LCG:
    def __init__(self, state: int):
        self.a = 1664525
        self.c = 1013904223
        self.m_mask = (2 << 30) - 1
        self.seed = state
        self.state = state

    @classmethod
    def from_seed(cls, seed: bytes) -> 'LCG':
        state = cls.cal_state(seed)
        return cls(state)

    @staticmethod
    def cal_state(seeds: bytes) -> int:
        acc = 0
        for byte in seeds:
            acc = (acc * 31 + byte) & 0xFFFFFFFF
        return acc

    def reset(self):
        self.state = self.seed

    def generate(self) -> int:
        self.state = (self.a * self.state + self.c) & self.m_mask
        return self.state

    def generate_u8(self) -> int:
        return self.generate() % 256

    def rand_range(self, start: int, end: int) -> int:
        return start + self.generate() % (end - start)

    def generate_random_string(self, length: int) -> str:
        CHARSET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        return ''.join(CHARSET[self.rand_range(0, len(CHARSET))] for _ in range(length))


def system_random() -> int:
    # 使用当前系统时间生成随机数
    now = int(time.time_ns() % (2 ** 32))
    rnd = LCG(now)
    return rnd.generate()
