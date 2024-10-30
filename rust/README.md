# CryptTool

Source Code: [https://github.com/guofei9987/CryptTool](https://github.com/guofei9987/CryptTool)

This project is a Tool Box about:
- Random number generator
- Cipher
- Byte Bits Converter

This project is mainly used in [https://github.com/guofei9987/hidden_watermark_rs](https://github.com/guofei9987/hidden_watermark_rs)  
or other projects.


# How to use

```rust
use crypt_tool::{system_random, BytesBitsConverter, LCG, XorCipher};
fn example_random() {
    let rand = system_random();
    println!("A random number = {}", rand);

    let seed = b"a seed";
    let mut rnd = LCG::from_seed(seed);
    let rand2 = rnd.generate_u8();
    println!("Another random number from seed {}", rand2);

    let random_str = rnd.generate_random_string(20);
    println!("A random String = {}", random_str);
}
fn example_cipher() {
    let cipher = XorCipher::new("password1".as_bytes());
    let data = vec![0, 255, 128, 64, 32, 16, 8, 4, 2, 1];
    let encoded = cipher.encode(&data);
    let decoded = cipher.decode(&encoded);
    assert_eq!(data, decoded);
}

fn example_data_bin_conversion() {
    let converter = BytesBitsConverter::new();
    let bytes = vec![0, 1, 2, 255];
    let bits = vec![
        0, 0, 0, 0, 0, 0, 0, 0, // 0
        0, 0, 0, 0, 0, 0, 0, 1, // 1
        0, 0, 0, 0, 0, 0, 1, 0, // 2
        1, 1, 1, 1, 1, 1, 1, 1, // 255
    ];
    assert_eq!(converter.bytes_to_bits(&bytes), bits);
    assert_eq!(converter.bits_to_bytes(&bits), bytes);
}

fn main() {
    example_random();
    example_cipher();
    example_data_bin_conversion();
}
```