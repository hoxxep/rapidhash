use criterion::{Bencher, Criterion};
use rand_core::{RngCore, SeedableRng};

pub fn bench(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("rng/rapidhash");
        group.bench_function("1", bench_rapidhash(1));
        group.bench_function("10000", bench_rapidhash(10000));
    }
    {
        let mut group = c.benchmark_group("rng/rapidhash_fast");
        group.bench_function("1", bench_rapidhash_fast(1));
        group.bench_function("10000", bench_rapidhash_fast(10000));
    }
    {
        let mut group = c.benchmark_group("rng/rapidhash_quality");
        group.bench_function("1", bench_rapidhash_quality(1));
        group.bench_function("10000", bench_rapidhash_quality(10000));
    }
    {
        let mut group = c.benchmark_group("rng/wyhash");
        group.bench_function("1", bench_wyhash(1));
        group.bench_function("10000", bench_wyhash(10000));
    }
}

pub fn bench_rapidhash(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut out = 0;
            let mut rng = rapidhash::RapidRng::seed_from_u64(i);
            for _ in 0..count {
                out ^= rng.next_u64();
            }
            (out, rng.next_u64())
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut out = 0;
            let mut rng = wyhash::WyRng::seed_from_u64(i);
            for _ in 0..count {
                out ^= rng.next_u64();
            }
            (out, rng.next_u64())
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_fast(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |mut i: u64| {
            let mut out = 0;
            for _ in 0..count {
                out ^= rapidhash::rapidrng_fast(&mut i);
            }
            (out, rapidhash::rapidrng_fast(&mut i))
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_quality(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random()
        }, |mut i: [u64; 3]| {
            let mut out: u64 = 0;
            for _ in 0..count {
                out ^= rapidhash::rapidrng_quality(&mut i);
            }
            (out, rapidhash::rapidrng_quality(&mut i))
        }, criterion::BatchSize::SmallInput);
    })
}
