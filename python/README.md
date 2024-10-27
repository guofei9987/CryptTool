# Crypt Tool

`crypt_tool` 提供了一个简单的加密和数据转换工具集，包含以下组件：

1. `LinearCongruentialGenerator`: 生成伪随机数，用于加密和字符串生成。
2. `XorCipher`: 基于 XOR 的简单加密解密类。
3. `BytesBitsConverter`: 将字节转换为位、位转换为字节的工具类。

## 使用示例

### 1. 生成随机数
```python
from crypt_tool import system_random, LinearCongruentialGenerator

rand = system_random()
print("A random number:", rand)

seed = b"a seed"
rnd = LinearCongruentialGenerator.from_seed(seed)
print("Random number from seed:", rnd.generate_u8())
print("Random string:", rnd.generate_random_string(20))
```