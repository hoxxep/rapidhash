use std::hash::Hasher;
use criterion::Bencher;
use rand::Rng;
use rand::rngs::OsRng;

pub fn bench_rapidhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_rapidhash_raw(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_default(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_fxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_t1ha(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_wyhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_wyhash_raw(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_xxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_metrohash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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

pub fn bench_seahash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
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
