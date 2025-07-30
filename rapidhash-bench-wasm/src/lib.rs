//! Simulate hashing fictional data structures in a WebAssembly environment.
//!
//! These benchmarks are slightly noisy, as the setup/teardown/loop overhead is contained within
//! the benchmark, just in case the WASM boundary calls are unreliably noisy.
//!
//! To reduce the noise as much as possible, the benchmark setup/teardown on each loop should not
//! allocate and drop repeatedly, as it introduces loads of noise and seems to skew some benchmarks
//! in ways I can't explain. Hence, the care to use stack-allocated buffers to simulate the variable
//! length string data.

use std::hash::BuildHasher;

use rapidhash::rng::rapidrng_fast_not_portable;

macro_rules! bench_wasm_tuple {
    ($name:ident, $hash:path) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name() -> u64 {
            profile_hash_tuple::<$hash>()
        }
    };
}

macro_rules! bench_wasm_4kb {
    ($name:ident, $hash:path) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name() -> u64 {
            profile_hash_4kb::<$hash>()
        }
    };
}

bench_wasm_tuple!(bench_wasm_rapidhash_q_tuple, rapidhash::quality::RandomState);
bench_wasm_tuple!(bench_wasm_rapidhash_f_tuple, rapidhash::fast::RandomState);
bench_wasm_tuple!(bench_wasm_foldhash_q_tuple, foldhash::quality::RandomState);
bench_wasm_tuple!(bench_wasm_foldhash_f_tuple, foldhash::fast::RandomState);
bench_wasm_tuple!(bench_wasm_default_tuple, std::hash::RandomState);
bench_wasm_tuple!(bench_wasm_fxhash_tuple, fxhash::FxBuildHasher);

bench_wasm_4kb!(bench_wasm_rapidhash_q_4kb, rapidhash::quality::RandomState);
bench_wasm_4kb!(bench_wasm_rapidhash_f_4kb, rapidhash::fast::RandomState);
bench_wasm_4kb!(bench_wasm_foldhash_q_4kb, foldhash::quality::RandomState);
bench_wasm_4kb!(bench_wasm_foldhash_f_4kb, foldhash::fast::RandomState);
bench_wasm_4kb!(bench_wasm_default_4kb, std::hash::RandomState);
bench_wasm_4kb!(bench_wasm_fxhash_4kb, fxhash::FxBuildHasher);

/// Simulate hashing fictional (id, email) pairs, where email is len 6..30 bytes.
fn profile_hash_tuple<B: BuildHasher + Default>() -> u64 {
    let builder = B::default();

    let mut seed = 0;
    let mut total = 0;

    let mut buffer = [0u64; 4];

    for _ in 0..1_000 {
        let ratio = f64::from(rapidrng_fast_not_portable(&mut seed) as u32) / f64::from(u32::MAX);
        let len = usize::max(6, (ratio * 30.0) as usize);

        buffer.fill_with(|| rapidrng_fast_not_portable(&mut seed));

        let username: &[u8] = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, 8 * buffer.len()) };
        let num = rapidrng_fast_not_portable(&mut seed);
        total ^= builder.hash_one((num, &username[..len]));
    }

    total
}

/// Simulate hashing a 3kb-4kb file or byte array.
fn profile_hash_4kb<B: BuildHasher + Default>() -> u64 {
    let builder = B::default();

    let mut seed = 0;
    let mut total = 0;

    let mut buffer = [0u64; 512];

    for _ in 0..1_000 {
        let ratio = f64::from(rapidrng_fast_not_portable(&mut seed) as u32) / f64::from(u32::MAX);
        let len = usize::min(usize::max(3 * 512, (ratio * 4.0 * 512.0) as usize), 512);
        buffer[0..len].fill_with(|| rapidrng_fast_not_portable(&mut seed));
        let file_bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, 8 * buffer.len()) };
        total ^= builder.hash_one(&file_bytes[..len * 8]);
    }

    total
}
