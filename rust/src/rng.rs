use std::ops::Range;
use std::time::SystemTime;

#[derive(Clone)]
pub struct LCG {
    a: u32,
    c: u32,
    m_mask: u32,
    seed: u32,
    state: u32,
}

impl LCG {
    fn from_state(state: u32) -> Self {
        let a: u32 = 1664525;
        let c: u32 = 1013904223;
        let m: u32 = 2u32 << 30;
        Self {
            a,
            c,
            m_mask: m - 1,
            seed: state,
            state,
        }
    }

    fn cal_state(seeds: &[u8]) -> u32 {
        seeds.iter().fold(0u32, |acc, &byte| {
            acc.wrapping_mul(31).wrapping_add(byte as u32)
        })
    }

    pub fn from_seed(seed: &[u8]) -> Self {
        Self::from_state(Self::cal_state(seed))
    }

    pub fn reset(&mut self) {
        self.state = self.seed;
    }

    pub fn generate(&mut self) -> u32 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) & self.m_mask;
        self.state
    }

    pub fn generate_u8(&mut self) -> u8 {
        (self.generate() % 256) as u8
    }

    pub fn rand_range(&mut self, range: Range<usize>) -> usize {
        let Range { start, end } = range;
        (self.generate() as usize) % (end - start) + start
    }

    pub fn generate_random_string(&mut self, len: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        (0..len)
            .map(|_| CHARSET[self.rand_range(0..CHARSET.len())] as char)
            .collect::<String>()
    }
}

pub fn system_random() -> u32 {
    // 根据系统时间来生成随机数
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() % u32::MAX as u128)
        .unwrap_or(0) as u32;

    let mut rnd = LCG::from_state(now);

    rnd.generate()
}
