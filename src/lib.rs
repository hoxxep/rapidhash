#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod hasher;
mod rapid;

pub use crate::hasher::*;
use crate::rapid::{rapidhash_raw, RAPID_SEED};

/// Rapidhash a single byte stream, matching the C++ implementation.
#[inline]
pub fn rapidhash(data: &[u8]) -> u64 {
    rapidhash_raw(data, RAPID_SEED)
}

/// Rapidhash a single bytestream, matching the C++ implementation, with a custom seed.
#[inline]
pub fn rapidhash_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_raw(data, seed)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::hash::Hasher;
    use std::collections::BTreeSet;
    use rand::Rng;
    use rand::rngs::OsRng;
    use super::*;

    #[test]
    fn it_works() {
        let hash = rapidhash(b"hello world");
        assert_ne!(hash, 0);
        assert_eq!(hash, 17498481775468162579);

        let mut hasher = RapidHasher::default();
        hasher.write(b"hello world");
        assert_eq!(hasher.finish(), 17498481775468162579);

        let hash = rapidhash(b"hello world!");
        assert_eq!(hash, 12238759925102402976);
    }

    /// Check RapidHasher is equivalent to the raw rapidhash for a single byte stream.
    ///
    /// Also check that the hash is unique for different byte streams.
    #[test]
    fn all_sizes() {
        let mut hashes = BTreeSet::new();

        for size in 0..=1024 {
            let mut data = std::vec![0; size];
            OsRng.fill(data.as_mut_slice());

            let hash1 = rapidhash(&data);
            let mut hasher = RapidHasher::default();
            hasher.write(&data);
            let hash2 = hasher.finish();

            assert_eq!(hash1, hash2, "Failed on size {}", size);
            assert!(!hashes.contains(&hash1), "Duplicate for size {}", size);

            hashes.insert(hash1);
        }
    }
}
