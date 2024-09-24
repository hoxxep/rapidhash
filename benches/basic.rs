use std::hash::Hasher;
use criterion::{Bencher, Criterion};
use rand::Rng;
use rand::rngs::OsRng;

/// Benchmark each hashing algorithm with various input sizes.
pub fn bench(c: &mut Criterion) {
    let groups: &[(&str, Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>)] = &[
        ("hash/rapidhash", Box::new(bench_rapidhash)),
        ("hash/rapidhash_raw", Box::new(bench_rapidhash_raw)),
        ("hash/default", Box::new(bench_default)),
        ("hash/fxhash", Box::new(bench_fxhash)),
        ("hash/t1ha", Box::new(bench_t1ha)),
        ("hash/wyhash", Box::new(bench_wyhash)),
        ("hash/wyhash_raw", Box::new(bench_wyhash_raw)),
        ("hash/xxhash", Box::new(bench_xxhash)),
        ("hash/metrohash", Box::new(bench_metrohash)),
        ("hash/seahash", Box::new(bench_seahash)),
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
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_rapidhash_raw(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

fn bench_xxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = xxhash_rust::xxh3::Xxh3::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_metrohash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = metrohash::MetroHash::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

fn bench_seahash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes: Vec<u8>| {
            let mut hasher = seahash::SeaHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}
