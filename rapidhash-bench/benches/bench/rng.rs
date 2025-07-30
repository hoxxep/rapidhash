use std::hash::BuildHasher;
use std::hint::black_box;
use criterion::{Bencher, Criterion};
use rand_core::{RngCore, SeedableRng};

macro_rules! bench_rng {
    ($c:ident, $name:literal, $function:ident) => {
        {
            let mut group = $c.benchmark_group(concat!("rng/", $name));
            group.throughput(criterion::Throughput::Elements(1));
            group.bench_function("1", $function(1));
            group.throughput(criterion::Throughput::Elements(10000));
            group.bench_function("10000", $function(10000));
        }
    };
}

pub fn bench(c: &mut Criterion) {
    bench_rng!(c, "rapidrng", bench_rapidhash);
    bench_rng!(c, "rapidrng_fast", bench_rapidhash_fast);
    bench_rng!(c, "rapidrng_fast_not_portable", bench_rapidhash_ultra_fast);
    bench_rng!(c, "rapidrng_time", bench_rapidhash_time);
    bench_rng!(c, "wyrng", bench_wyhash);

    let mut group = c.benchmark_group("random_state");
    group.bench_function("random_state", bench_state::<rapidhash::quality::RandomState>);
    group.bench_function("build_hasher", bench_state::<rapidhash::quality::RapidBuildHasher>);
}

pub fn bench_rapidhash(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut out = 0;
            let mut rng = rapidhash::rng::RapidRng::seed_from_u64(i);
            for _ in 0..count {
                out ^= rng.next_u64();
            }
            out
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
                out ^= rapidhash::rng::rapidrng_fast(&mut i);
            }
            out
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_ultra_fast(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |mut i: u64| {
            let mut out = 0;
            for _ in 0..count {
                out ^= rapidhash::rng::rapidrng_fast_not_portable(&mut i);
            }
            out
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_time(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random()
        }, |mut i: u64| {
            let mut out: u64 = 0;
            for _ in 0..=count {
                out ^= rapidhash::rng::rapidrng_time(&mut i);
            }
            out
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
            out
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_state<T: BuildHasher + Default>(b: &mut Bencher) {
    b.iter_batched(|| {
        ()
    }, |()| {
        black_box(T::default())
    }, criterion::BatchSize::SmallInput);
}
