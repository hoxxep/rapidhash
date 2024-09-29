use std::hash::Hasher;
use criterion::Bencher;
use rand::Rng;
use rand::rngs::OsRng;
use rapidhash::RAPID_SEED;

/// Use .iter_batched_ref to avoid paying the Vec destruction cost, as it's 10x
/// more expensive than our small benchmarks!!
pub fn bench_rapidhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_raw(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            rapidhash::rapidhash_inline(&bytes, RAPID_SEED)
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_default(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_fxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = fxhash::FxHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_t1ha(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = t1ha::T1haHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = wyhash::WyHash::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash_raw(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            wyhash::wyhash(&bytes, 0)
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_xxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = xxhash_rust::xxh3::Xxh3::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_metrohash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = metrohash::MetroHash::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_seahash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = seahash::SeaHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_ahash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = ahash::AHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_gxhash(size: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; size];
            OsRng.fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            let mut hasher = gxhash::GxHasher::default();
            hasher.write(&bytes);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}
