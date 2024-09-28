#![no_main]

use std::hash::Hasher;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mut hasher = rapidhash::RapidHasher::default();
    hasher.write(data);
    let _ = hasher.finish();
});
