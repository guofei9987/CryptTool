use super::rng::LCG;

pub struct XorCipher {
    rng: LCG,
}

impl XorCipher {
    pub fn new(pwd: &[u8]) -> Self {
        Self {
            rng: LCG::from_seed(pwd),
        }
    }

    pub fn apply_xor(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = self.rng.clone();
        data.iter().map(|byte1| byte1 ^ rng.generate_u8()).collect()
    }

    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        self.apply_xor(data)
    }
    pub fn decode(&self, data: &[u8]) -> Vec<u8> {
        self.apply_xor(data)
    }
}
