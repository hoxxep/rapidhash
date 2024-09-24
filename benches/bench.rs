use criterion::{criterion_group, criterion_main};

mod basic;
mod hashmap;

criterion_group!(
    benches,
    basic::bench,
    hashmap::bench,
);
criterion_main!(benches);
