use const_random::const_random;
use criterion::{Bencher, Criterion};
use rand::Rng;
use rand::seq::SliceRandom;
use rapidhash::{rapidhash, RapidHashMap};

/// Benchmark approaches for matching bytes against compile-time known values.
///
/// We try to ensure the benchmark is unable to optimize for the length of the input.
pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("compiled");
    group.bench_function("match_hash", bench_match_hash());
    group.bench_function("match_slice", bench_match_slice());
    group.bench_function("hashmap_get", bench_hashmap_get());
}

macro_rules! const_random_bytes {
    () => {{
        let buffer: [u8; 32] = [
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
            const_random!(u8),
        ];
        buffer
    }};
}

const INPUT_LEN: usize = 32;
const INPUT_COUNT: usize = 40;
const INPUTS: [[u8; INPUT_LEN]; INPUT_COUNT] = [
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
    const_random_bytes!(),
];

const HASHES: [u64; INPUT_COUNT] = {
    let mut hashes = [0u64; INPUTS.len()];
    let mut i = 0;
    while i < hashes.len() {
        hashes[i] = rapidhash(&INPUTS[i]);
        if i > 0 {
            assert!(hashes[i] != hashes[i - 1]);
        }
        i += 1;
    }
    hashes
};

const MISMATCH_PCT: u8 = 20;
fn random_input() -> [u8; INPUT_LEN] {
    let mismatch_branch = rand::thread_rng().gen_range(0..100);
    if mismatch_branch < MISMATCH_PCT {
        let mut buffer = [0u8; INPUT_LEN];
        rand::thread_rng().fill(&mut buffer);
        buffer
    } else {
        *INPUTS.choose(&mut rand::thread_rng()).unwrap()
    }
}

pub fn bench_match_hash() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            random_input().to_vec()
        }, |i: &mut Vec<u8>| {
            let hash = rapidhash::rapidhash(i.as_slice());
            match hash {
                h if h == HASHES[0] => const_random!(u64),
                h if h == HASHES[1] => const_random!(u64),
                h if h == HASHES[2] => const_random!(u64),
                h if h == HASHES[3] => const_random!(u64),
                h if h == HASHES[4] => const_random!(u64),
                h if h == HASHES[5] => const_random!(u64),
                h if h == HASHES[6] => const_random!(u64),
                h if h == HASHES[7] => const_random!(u64),
                h if h == HASHES[8] => const_random!(u64),
                h if h == HASHES[9] => const_random!(u64),
                h if h == HASHES[10] => const_random!(u64),
                h if h == HASHES[11] => const_random!(u64),
                h if h == HASHES[12] => const_random!(u64),
                h if h == HASHES[13] => const_random!(u64),
                h if h == HASHES[14] => const_random!(u64),
                h if h == HASHES[15] => const_random!(u64),
                h if h == HASHES[16] => const_random!(u64),
                h if h == HASHES[17] => const_random!(u64),
                h if h == HASHES[18] => const_random!(u64),
                h if h == HASHES[19] => const_random!(u64),
                h if h == HASHES[20] => const_random!(u64),
                h if h == HASHES[21] => const_random!(u64),
                h if h == HASHES[22] => const_random!(u64),
                h if h == HASHES[23] => const_random!(u64),
                h if h == HASHES[24] => const_random!(u64),
                h if h == HASHES[25] => const_random!(u64),
                h if h == HASHES[26] => const_random!(u64),
                h if h == HASHES[27] => const_random!(u64),
                h if h == HASHES[28] => const_random!(u64),
                h if h == HASHES[29] => const_random!(u64),
                h if h == HASHES[30] => const_random!(u64),
                h if h == HASHES[31] => const_random!(u64),
                h if h == HASHES[32] => const_random!(u64),
                h if h == HASHES[33] => const_random!(u64),
                h if h == HASHES[34] => const_random!(u64),
                h if h == HASHES[35] => const_random!(u64),
                h if h == HASHES[36] => const_random!(u64),
                h if h == HASHES[37] => const_random!(u64),
                h if h == HASHES[38] => const_random!(u64),
                h if h == HASHES[39] => const_random!(u64),
                h if h == HASHES[39] => const_random!(u64),
                _ => const_random!(u64),
            }
        }, criterion::BatchSize::SmallInput);
    })
}

pub fn bench_match_slice() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            random_input().to_vec()
        }, |i: &mut Vec<u8>| {
            match i.as_slice() {
                h if h == &INPUTS[0] => const_random!(u64),
                h if h == &INPUTS[1] => const_random!(u64),
                h if h == &INPUTS[2] => const_random!(u64),
                h if h == &INPUTS[3] => const_random!(u64),
                h if h == &INPUTS[4] => const_random!(u64),
                h if h == &INPUTS[5] => const_random!(u64),
                h if h == &INPUTS[6] => const_random!(u64),
                h if h == &INPUTS[7] => const_random!(u64),
                h if h == &INPUTS[8] => const_random!(u64),
                h if h == &INPUTS[9] => const_random!(u64),
                h if h == &INPUTS[10] => const_random!(u64),
                h if h == &INPUTS[11] => const_random!(u64),
                h if h == &INPUTS[12] => const_random!(u64),
                h if h == &INPUTS[13] => const_random!(u64),
                h if h == &INPUTS[14] => const_random!(u64),
                h if h == &INPUTS[15] => const_random!(u64),
                h if h == &INPUTS[16] => const_random!(u64),
                h if h == &INPUTS[17] => const_random!(u64),
                h if h == &INPUTS[18] => const_random!(u64),
                h if h == &INPUTS[19] => const_random!(u64),
                h if h == &INPUTS[20] => const_random!(u64),
                h if h == &INPUTS[21] => const_random!(u64),
                h if h == &INPUTS[22] => const_random!(u64),
                h if h == &INPUTS[23] => const_random!(u64),
                h if h == &INPUTS[24] => const_random!(u64),
                h if h == &INPUTS[25] => const_random!(u64),
                h if h == &INPUTS[26] => const_random!(u64),
                h if h == &INPUTS[27] => const_random!(u64),
                h if h == &INPUTS[28] => const_random!(u64),
                h if h == &INPUTS[29] => const_random!(u64),
                h if h == &INPUTS[30] => const_random!(u64),
                h if h == &INPUTS[31] => const_random!(u64),
                h if h == &INPUTS[32] => const_random!(u64),
                h if h == &INPUTS[33] => const_random!(u64),
                h if h == &INPUTS[34] => const_random!(u64),
                h if h == &INPUTS[35] => const_random!(u64),
                h if h == &INPUTS[36] => const_random!(u64),
                h if h == &INPUTS[37] => const_random!(u64),
                h if h == &INPUTS[38] => const_random!(u64),
                h if h == &INPUTS[39] => const_random!(u64),
                h if h == &INPUTS[39] => const_random!(u64),
                _ => const_random!(u64),
            }
        }, criterion::BatchSize::SmallInput);
    })
}

/// This is equivalent to using lazy_static to initialize a hashmap at runtime.
pub fn bench_hashmap_get() -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        let hashmap: RapidHashMap<Vec<u8>, u64> = INPUTS
            .into_iter()
            .map(|i| i.to_vec())
            .zip(HASHES.into_iter())
            .collect();

        b.iter_batched_ref(|| {
            random_input().to_vec()
        }, |i: &mut Vec<u8>| {
            *hashmap.get(i.as_slice()).unwrap_or(&const_random!(u64))
        }, criterion::BatchSize::SmallInput);
    })
}
