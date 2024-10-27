#[cfg(test)]
mod tests {
    use crypt_tool::{system_random, BytesBitsConverter, LinearCongruentialGenerator, XorCipher};

    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_get_true_rand() {
        let rand1 = system_random();
        thread::sleep(Duration::from_millis(100));
        let rand2 = system_random();
        // 由于依赖于系统时间，通常不相等
        assert_ne!(rand1, rand2);
    }

    #[test]
    fn test_rnd_gen_u8() {
        let seed = b"another_seed";
        let mut rnd = LinearCongruentialGenerator::from_seed(seed);

        // 生成一个 u32 随机数，然后取模 256
        let generated_u8_1 = rnd.generate_u8();

        let generated_u8_2 = (rnd.generate() % 256) as u8;

        assert_ne!(generated_u8_1, generated_u8_2);
        let generated_u8_3 = rnd.generate_u8();
        assert_ne!(generated_u8_2, generated_u8_3);

        rnd.reset();
        let generated_u8_4 = rnd.generate_u8();
        assert_eq!(generated_u8_1, generated_u8_4);
    }

    #[test]
    fn test_rnd_zero_seed() {
        // 测试种子为全零的情况
        let seed = &[0u8; 8];
        let mut rnd = LinearCongruentialGenerator::from_seed(seed);

        let first = rnd.generate();
        let second = rnd.generate();
        assert_ne!(first, second);
    }

    #[test]
    fn test_rnd_no_seed() {
        // 测试种子为空的情况
        let seed = &[0u8; 0];
        let mut rnd = LinearCongruentialGenerator::from_seed(seed);

        let first = rnd.generate();
        let second = rnd.generate();
        assert_ne!(first, second);
    }

    #[test]
    fn test_generate_random_string_length() {
        let mut rnd = LinearCongruentialGenerator::from_seed("password2".as_bytes());
        let random_str = rnd.generate_random_string(50);
        assert_eq!(random_str.len(), 50);
    }

    #[test]
    fn test_random_string() {
        let mut rnd1 = LinearCongruentialGenerator::from_seed("pwd3".as_bytes());
        let mut rnd2 = LinearCongruentialGenerator::from_seed("pwd3".as_bytes());

        let str1 = rnd1.generate_random_string(100);
        let str2 = rnd2.generate_random_string(100);
        // 由于使用相同的种子，生成的字符串应相同
        assert_eq!(str1, str2);
    }

    #[test]
    fn test_random_string2() {
        let mut rnd = LinearCongruentialGenerator::from_seed("密码".as_bytes());

        let res1 = rnd.generate_random_string(20);
        println!("生成随机字符：{}", res1);
        assert_eq!(res1, "xOHsZo7o7Sfe9eTOJsly");
    }

    #[test]
    fn test_cipher_encode_decode() {
        let cipher = XorCipher::new("password1".as_bytes());
        let data = vec![0, 255, 128, 64, 32, 16, 8, 4, 2, 1];
        let encoded = cipher.encode(&data);
        let decoded = cipher.decode(&encoded);
        assert_eq!(data, decoded);
        let encoded1 = cipher.encode(&data);
        assert_eq!(encoded1, encoded);
    }

    #[test]
    fn test_data_bin_conversion() {
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
}
