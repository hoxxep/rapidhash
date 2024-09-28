use criterion::{criterion_group, criterion_main};

mod basic;
mod int;
mod vector;
mod object;
mod hashmap;
mod rng;
mod compiled;

criterion_group!(
    benches,
    basic::bench,
    hashmap::bench,
    rng::bench,
    compiled::bench,
);
criterion_main!(benches);
