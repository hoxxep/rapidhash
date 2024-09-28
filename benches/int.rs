use std::hash::Hasher;
use criterion::Bencher;

pub fn bench_rapidhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_u8() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random()
        }, |i| {
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write_u8(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_u16() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random()
        }, |i| {
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write_u16(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_u32() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random()
        }, |i| {
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write_u32(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_u128() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random()
        }, |i| {
            let mut hasher = rapidhash::RapidHasher::default();
            hasher.write_u128(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_raw() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            rapidhash::rapidhash(i.to_le_bytes().as_slice())
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_default() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_fxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = fxhash::FxHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_t1ha() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = t1ha::T1haHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = wyhash::WyHash::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash_raw() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            wyhash::wyhash(i.to_le_bytes().as_slice(), 0)
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_xxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = xxhash_rust::xxh3::Xxh3::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_metrohash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = metrohash::MetroHash::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_seahash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = seahash::SeaHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_ahash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = ahash::AHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_gxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            rand::random::<u64>()
        }, |i: u64| {
            let mut hasher = gxhash::GxHasher::default();
            hasher.write_u64(i);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}
