# Crypt Tool

`crypt_tool` 提供了一个简单的加密和数据转换工具集，包含以下组件：

1. `LinearCongruentialGenerator`: 生成伪随机数，用于加密和字符串生成。
2. `XorCipher`: 基于 XOR 的简单加密解密类。
3. `BytesBitsConverter`: 将字节转换为位、位转换为字节的工具类。

## 使用示例

```shell
# (cd ./python)
# 运行示例函数
python examples/example.py

# 批量测试
python tests/tests.py --verbose
```

### 1. 生成随机数

```python
from crypt_tool import system_random, LCG

rand = system_random()
print("A random number:", rand)

seed = b"a seed"
rnd = LCG.from_seed(seed)
print("Random number from seed:", rnd.generate_u8())
print("Random string:", rnd.generate_random_string(20))
```

### 2. 加密

```python
from crypt_tool import XorCipher

cipher = XorCipher(b"password1")
data = bytes([0, 255, 128, 64, 32, 16, 8, 4, 2, 1])
encoded = cipher.encode(data)
decoded = cipher.decode(encoded)
assert data == decoded, "Decoded data does not match original"
print("Encoding and decoding successful.")
```

### 3. 二进制类型 和 Bytes 类型互相转化
```python
from crypt_tool import BytesBitsConverter

converter = BytesBitsConverter()
bytes_data = bytes([0, 1, 2, 255])

bits = [
    0, 0, 0, 0, 0, 0, 0, 0,  # 0
    0, 0, 0, 0, 0, 0, 0, 1,  # 1
    0, 0, 0, 0, 0, 0, 1, 0,  # 2
    1, 1, 1, 1, 1, 1, 1, 1,  # 255
]

assert converter.bytes_to_bits(bytes_data) == bits
assert bytes_data == converter.bits_to_bytes(bits)
```

