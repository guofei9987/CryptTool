import unittest
from crypt_tool import LCG, system_random, XorCipher, BytesBitsConverter, CryptConverter
import time


class TestCryptTool(unittest.TestCase):
    def test_get_true_rand(self):
        rand1 = system_random()
        time.sleep(0.1)
        rand2 = system_random()
        self.assertNotEqual(rand1, rand2)

    def test_rnd_gen_u8(self):
        seed = b"another_seed"
        rnd = LCG.from_seed(seed)
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
        rnd = LCG.from_seed(seed)
        self.assertNotEqual(rnd.generate(), rnd.generate())

    def test_rnd_no_seed(self):
        seed = bytes()
        rnd = LCG.from_seed(seed)
        self.assertNotEqual(rnd.generate(), rnd.generate())

    def test_generate_random_string_length(self):
        rnd = LCG.from_seed(b"password2")
        random_str = rnd.generate_random_string(50)
        self.assertEqual(len(random_str), 50)

    def test_random_string(self):
        rnd1 = LCG.from_seed(b"pwd3")
        rnd2 = LCG.from_seed(b"pwd3")
        str1 = rnd1.generate_random_string(100)
        str2 = rnd2.generate_random_string(100)
        self.assertEqual(str1, str2)

    def test_cipher_encode_decode(self):
        cipher = XorCipher(b"password1")
        data = bytes([0, 255, 128, 64, 32, 16, 8, 4, 2, 1])
        data_encodes = bytes([221, 103, 151, 202, 65, 92, 51, 90, 39, 65])

        self.assertEqual(data, cipher.decode(data_encodes))
        self.assertEqual(cipher.encode(data), data_encodes)

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
        self.assertEqual(bits, converter.bytes_to_bits(bytes_data))

    def test_crypt_converter(self):
        crypt_converter = CryptConverter(b"pwd")
        bytes_data = bytes([0, 1, 2, 255])

        bytes_encode = crypt_converter.encode(bytes_data)

        self.assertEqual(bytes_data, crypt_converter.decode(bytes_encode))
        self.assertEqual(bytes_encode, crypt_converter.encode(bytes_data))


if __name__ == "__main__":
    unittest.main()
