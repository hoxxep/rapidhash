use criterion::{criterion_group, criterion_main};

mod basic;
mod int;
mod string;
mod object;
mod hashmap;

criterion_group!(
    benches,
    basic::bench,
    // string::bench,
    hashmap::bench,
);
criterion_main!(benches);
