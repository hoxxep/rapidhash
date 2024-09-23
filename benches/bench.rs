use criterion::{criterion_group, criterion_main};

mod basic;

criterion_group!(
    benches,
    basic::bench,
);
criterion_main!(benches);
