use crypt_tool::{simple_random, BytesBitsConverter, XorCipher, LCG};
fn example_random() {
    let rand = simple_random();
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
