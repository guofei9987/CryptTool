import unittest
from crypt_tool import LinearCongruentialGenerator, system_random, XorCipher, BytesBitsConverter
import time


class TestCryptTool(unittest.TestCase):
    def test_get_true_rand(self):
        rand1 = system_random()
        time.sleep(0.1)
        rand2 = system_random()
        self.assertNotEqual(rand1, rand2)

    def test_rnd_gen_u8(self):
        seed = b"another_seed"
        rnd = LinearCongruentialGenerator.from_seed(seed)
        generated_u8_1 = rnd.generate_u8()
        generated_u8_2 = rnd.generate() % 256
        self.assertNotEqual(generated_u8_1, generated_u8_2)
        generated_u8_3 = rnd.generate_u8()
        self.assertNotEqual(generated_u8_2, generated_u8_3)

        rnd.reset()
        generated_u8_4 = rnd.generate_u8()
        self.assertEqual(generated_u8_1, generated_u8_4)

    def test_rnd_zero_seed(self):
        seed = bytes([0] * 8)
        rnd = LinearCongruentialGenerator.from_seed(seed)
        self.assertNotEqual(rnd.generate(), rnd.generate())

    def test_rnd_no_seed(self):
        seed = bytes()
        rnd = LinearCongruentialGenerator.from_seed(seed)
        self.assertNotEqual(rnd.generate(), rnd.generate())

    def test_generate_random_string_length(self):
        rnd = LinearCongruentialGenerator.from_seed(b"password2")
        random_str = rnd.generate_random_string(50)
        self.assertEqual(len(random_str), 50)

    def test_random_string(self):
        rnd1 = LinearCongruentialGenerator.from_seed(b"pwd3")
        rnd2 = LinearCongruentialGenerator.from_seed(b"pwd3")
        str1 = rnd1.generate_random_string(100)
        str2 = rnd2.generate_random_string(100)
        self.assertEqual(str1, str2)

    def test_cipher_encode_decode(self):
        cipher = XorCipher(b"password1")
        data = bytes([0, 255, 128, 64, 32, 16, 8, 4, 2, 1])
        encoded = cipher.encode(data)
        decoded = cipher.decode(encoded)
        self.assertEqual(data, decoded)

    def test_data_bin_conversion(self):
        converter = BytesBitsConverter()
        bytes_data = bytes([0, 1, 2, 255])
        bits = converter.bytes_to_bits(bytes_data)
        self.assertEqual(bits, [
            0, 0, 0, 0, 0, 0, 0, 0,  # 0
            0, 0, 0, 0, 0, 0, 0, 1,  # 1
            0, 0, 0, 0, 0, 0, 1, 0,  # 2
            1, 1, 1, 1, 1, 1, 1, 1  # 255
        ])
        self.assertEqual(converter.bits_to_bytes(bits), bytes_data)


if __name__ == "__main__":
    unittest.main()
