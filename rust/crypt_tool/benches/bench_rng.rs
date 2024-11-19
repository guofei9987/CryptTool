use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crypt_tool::{system_random, LCG};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

// 使用 LinearCongruentialGenerator 生成随机数
fn bench_custom_rng(c: &mut Criterion) {
    let seed = b"some_seed";
    let mut rng = LCG::from_seed(seed);

    c.bench_function("custom_rng_generate", |b| {
        b.iter(|| {
            let _ = black_box(rng.generate());
        })
    });
}

// 使用 rand 库的 StdRng 生成随机数
fn bench_rand_rng(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(system_random() as u64);

    c.bench_function("rand_rng_generate", |b| {
        b.iter(|| {
            let _ = black_box(rng.gen::<u32>());
        })
    });
}

criterion_group!(benches, bench_custom_rng, bench_rand_rng);
criterion_main!(benches);
