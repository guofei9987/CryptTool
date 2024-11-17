from crypt_tool import LCG, system_random, XorCipher, BytesBitsConverter


def example_random():
    rand = system_random()
    print(f"A random number = {rand}")

    seed = b"a seed"
    rnd = LCG.from_seed(seed)
    rand2 = rnd.generate_u8()
    print(f"Another random number from seed = {rand2}")

    random_str = rnd.generate_random_string(20)
    print(f"A random String = {random_str}")


def example_cipher():
    cipher = XorCipher(b"password1")
    data = bytes([0, 255, 128, 64, 32, 16, 8, 4, 2, 1])
    encoded = cipher.encode(data)
    decoded = cipher.decode(encoded)
    assert data == decoded, "Decoded data does not match original"
    print("Encoding and decoding successful.")


def example_data_bin_conversion():
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


if __name__ == "__main__":
    example_random()
    example_cipher()
    example_data_bin_conversion()
