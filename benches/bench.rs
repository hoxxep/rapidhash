use criterion::{criterion_group, criterion_main};

mod basic;
mod int;
mod vector;
mod object;
mod hashmap;
mod rng;

criterion_group!(
    benches,
    basic::bench,
    hashmap::bench,
    rng::bench,
);
criterion_main!(benches);
