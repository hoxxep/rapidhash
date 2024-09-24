use std::hash::Hasher;
use criterion::{Bencher, Criterion};
use rand::Rng;
use rand::rngs::OsRng;

/// Benchmark each hashing algorithm with various input sizes.
pub fn bench(c: &mut Criterion) {
    let groups: &[(&str, Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>)] = &[
        ("rapidhash", Box::new(bench_rapidhash)),
        ("default", Box::new(bench_default)),
        ("fxhash", Box::new(bench_fxhash)),
        ("t1ha", Box::new(bench_t1ha)),
        ("wyhash", Box::new(bench_wyhash)),
        ("wyhash_raw", Box::new(bench_wyhash_raw)),
    ];

    let sizes = [8usize, 16, 64, 256, 1024, 4096];

    for (name, function) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for size in sizes {
            group.bench_function(size.to_string(), function(size));
        }
    }
}

fn bench_rapidhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            rapidhash::rapidhash(&bytes)
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_default(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_fxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = fxhash::FxHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_t1ha(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = t1ha::T1haHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_wyhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = wyhash::WyHash::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_wyhash_raw(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            wyhash::wyhash(&bytes, 0)
        }, criterion::BatchSize::SmallInput);
    })
}
