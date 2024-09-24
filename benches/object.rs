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

pub fn bench_rapidhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = rapidhash::RapidHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_default() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_fxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = fxhash::FxHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_t1ha() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = t1ha::T1haHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_wyhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = wyhash::WyHash::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_xxhash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = xxhash_rust::xxh3::Xxh3::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_metrohash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = metrohash::MetroHash::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_seahash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = seahash::SeaHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_ahash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            Object::random()
        }, |o: Object| {
            let mut hasher = ahash::AHasher::default();
            o.hash(&mut hasher);
            hasher.finish()
        }, criterion::BatchSize::SmallInput);
    })
}
