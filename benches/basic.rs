use std::hash::Hasher;
use criterion::{Bencher, Criterion};
use rand::Rng;
use rand::rngs::OsRng;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic");
    group.bench_function("rapidhash_8", bench_rapidhash(8));
    group.bench_function("rapidhash_16", bench_rapidhash(16));
    group.bench_function("rapidhash_64", bench_rapidhash(64));
    group.bench_function("rapidhash_256", bench_rapidhash(256));
    group.bench_function("rapidhash_1024", bench_rapidhash(1024));

    group.bench_function("fxhash_8", bench_fxhash(8));
    group.bench_function("fxhash_16", bench_fxhash(16));
    group.bench_function("fxhash_64", bench_fxhash(64));
    group.bench_function("fxhash_256", bench_fxhash(256));
    group.bench_function("fxhash_1024", bench_fxhash(1024));

    group.bench_function("default_8", bench_default(8));
    group.bench_function("default_16", bench_default(16));
    group.bench_function("default_64", bench_default(64));
    group.bench_function("default_256", bench_default(256));
    group.bench_function("default_1024", bench_default(1024));
}

fn bench_rapidhash(size: usize) -> impl FnMut(&mut Bencher) {
    move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            rapidhash::rapidhash(&bytes)
        }, criterion::BatchSize::SmallInput);
    }
}

fn bench_default(size: usize) -> impl FnMut(&mut Bencher) {
    move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    }
}


fn bench_fxhash(size: usize) -> impl FnMut(&mut Bencher) {
    move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = fxhash::FxHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    }
}
