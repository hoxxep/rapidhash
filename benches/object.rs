use std::hash::{Hash, Hasher};
use criterion::{Bencher};

#[derive(Hash)]
struct Object {
    a: u8,
    b: u64,
    s: String,
    v: Vec<u32>,
}

impl Object {
    pub fn random() -> Self {
        Object {
            a: rand::random(),
            b: rand::random(),
            s: rand::random::<u64>().to_string(),
            v: vec![rand::random(), rand::random(), rand::random()],
        }
    }
}

/// Use .iter_batched_ref to avoid paying the Object destruction cost, as it's 10x
/// more expensive than our small benchmarks!!
pub fn bench_rapidhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = rapidhash::RapidHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rapidhash_inline() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = rapidhash::RapidInlineHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_default() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_fxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = fxhash::FxHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_t1ha() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = t1ha::T1haHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = wyhash::WyHash::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_xxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = twox_hash::XxHash::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_metrohash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = metrohash::MetroHash::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_seahash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = seahash::SeaHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_ahash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = ahash::AHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_gxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = gxhash::GxHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_farmhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = farmhash::FarmHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_highwayhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = highway::HighwayHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_rustchash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            Object::random()
        }, |o| {
            let mut hasher = rustc_hash::FxHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}
