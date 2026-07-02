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

use rand::{Rng, RngExt, SeedableRng};
use rapidrand::RapidRng;

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

/// Hash a fixed value with a fresh `RandomState`, used to test per-map seed uniqueness and
/// cross-instance determinism on wasm.
#[unsafe(no_mangle)]
pub extern "C" fn test_wasm_random_state() -> u64 {
    rapidhash::fast::RandomState::default().hash_one(42u64)
}

/// Hash a fixed value with the process-wide `GlobalState`, used to test that the one-time
/// global seed/secret initialization works on wasm.
#[unsafe(no_mangle)]
pub extern "C" fn test_wasm_global_state() -> u64 {
    rapidhash::fast::GlobalState::default().hash_one(42u64)
}

/// Simulate hashing fictional (id, email) pairs, where email is len 6..60 bytes.
fn profile_hash_tuple<B: BuildHasher + Default>() -> u64 {
    let builder = B::default();
    let mut rng = RapidRng::seed_from_u64(0);

    let mut total = 0;
    let mut buffer = [0u8; 60];

    for _ in 0..1_000 {
        rng.fill_bytes(&mut buffer);

        let len = rng.random_range(6..60);
        let num: u64 = rng.random();
        total ^= builder.hash_one((num, &buffer[..len]));
    }

    total
}

/// Simulate hashing a 3kb-4kb file or byte array.
fn profile_hash_4kb<B: BuildHasher + Default>() -> u64 {
    let builder = B::default();
    let mut rng = RapidRng::seed_from_u64(0);

    let mut total = 0;
    let mut buffer = [0u8; 4096];

    for _ in 0..1_000 {
        rng.fill_bytes(&mut buffer);

        let len = rng.random_range(3 * 1024..4 * 1024);
        total ^= builder.hash_one(&buffer[..len]);
    }

    total
}

/// Basic validity test of this code for non-WASM builds
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_hash_tuple() {
        let total = profile_hash_tuple::<rapidhash::fast::RandomState>();
        assert_ne!(total, 0);
    }

    #[test]
    fn test_profile_hash_4kb() {
        let total = profile_hash_tuple::<rapidhash::fast::RandomState>();
        assert_ne!(total, 0);
    }
}
