#[cfg(test)]
mod tests {
    use crypt_tool::{
        simple_random, system_random, BytesBitsConverter, CryptConverter, XorCipher, LCG,
    };

    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_system_rand() {
        let rand1 = system_random();
        thread::sleep(Duration::from_millis(100));
        let rand2 = system_random();
        // 由于依赖于系统时间，通常不相等
        assert_ne!(rand1, rand2);
    }
    #[test]
    fn test_simple_rand() {
        let rand1 = simple_random();
        let rand2 = simple_random();
        assert_ne!(rand1, rand2);
    }

    #[test]
    fn test_rnd_gen_u32() {
        let mut rnd = LCG::from_seed(b"seed");
        let generated_1 = rnd.generate();
        let generated_2 = rnd.generate();
        rnd.reset();
        let generated_3 = rnd.generate();
        assert_ne!(generated_1, generated_2);
        assert_eq!(generated_1, generated_3);
    }

    #[test]
    fn test_rnd_gen_u8() {
        let mut rnd = LCG::from_seed(b"seed2");
        assert_ne!(rnd.generate(), rnd.generate());
    }

    #[test]
    fn test_rnd_zero_seed() {
        // 测试种子为全零的情况
        let seed = &[0u8; 8];
        let mut rnd = LCG::from_seed(seed);
        assert_ne!(rnd.generate(), rnd.generate());
    }

    #[test]
    fn test_rnd_no_seed() {
        // 测试种子为空的情况
        let seed = &[0u8; 0];
        let mut rnd = LCG::from_seed(seed);
        assert_ne!(rnd.generate(), rnd.generate());
    }

    #[test]
    fn test_generate_random_string_length() {
        let mut rnd = LCG::from_seed("password2".as_bytes());
        let random_str = rnd.generate_random_string(50);
        assert_eq!(random_str.len(), 50);
    }

    #[test]
    fn test_random_string() {
        let mut rnd1 = LCG::from_seed("pwd3".as_bytes());
        let mut rnd2 = LCG::from_seed("pwd3".as_bytes());

        let str1 = rnd1.generate_random_string(100);
        let str2 = rnd2.generate_random_string(100);
        // 由于使用相同的种子，生成的字符串应相同
        assert_eq!(str1, str2);
    }

    #[test]
    fn test_random_string2() {
        let mut rnd = LCG::from_seed("密码".as_bytes());

        let res1 = rnd.generate_random_string(20);
        assert_eq!(res1, "xOHsZo7o7Sfe9eTOJsly");
    }

    #[test]
    fn test_rand_range() {
        let mut rnd = LCG::from_seed("pwd".as_bytes());

        let first = rnd.rand_range(0..20);
        assert!(first < 20);

        let second = rnd.rand_range(0..1);
        assert_eq!(second, 0);
    }

    #[test]
    fn test_cipher_encode_decode() {
        let cipher = XorCipher::new("password1".as_bytes());
        let data = vec![0, 255, 128, 64, 32, 16, 8, 4, 2, 1];
        let data_encodes = vec![221, 103, 151, 202, 65, 92, 51, 90, 39, 65];
        assert_eq!(data, cipher.decode(&data_encodes));
        assert_eq!(cipher.encode(&data), data_encodes);
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

    #[test]
    fn test_crypt_converter() {
        let crypt_converter = CryptConverter::new("pwd".as_bytes());
        let bytes = vec![0, 1, 2, 255];

        let bytes_encode = crypt_converter.encode(&bytes);

        assert_eq!(bytes, crypt_converter.decode(&bytes_encode));
        assert_eq!(bytes_encode, crypt_converter.encode(&bytes));
    }
}
