use criterion::{criterion_group, criterion_main};

mod basic;
mod compiled;
mod emails;
mod rng;
mod seed_path;
mod state;
mod streaming;

criterion_group!(
    benches,
    basic::bench,
    emails::bench,
    rng::bench,
    compiled::bench,
    seed_path::bench,
    state::bench,
    streaming::bench,
);
criterion_main!(benches);
