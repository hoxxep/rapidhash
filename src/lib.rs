#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod hasher;
mod rapid;
#[cfg(feature = "rand")]
mod random;

#[doc(inline)]
pub use crate::hasher::*;

use crate::rapid::{rapidhash_raw, RAPID_SEED};

#[doc(inline)]
#[cfg(feature = "rand")]
pub use crate::random::*;

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

    #[test]
    fn it_hashes_as_expected() {
        assert_eq!(                                    rapidhash(0u128.to_le_bytes().as_slice()),  8755926293314635566);
        assert_eq!(                                    rapidhash(1u128.to_le_bytes().as_slice()), 17996969877019643443);
        assert_eq!(                               rapidhash(0x1000u128.to_le_bytes().as_slice()),  3752997491443908878);
        assert_eq!(                          rapidhash(0x1000_0000u128.to_le_bytes().as_slice()),  1347028408682550078);
        assert_eq!(                  rapidhash(0x10000000_00000000u128.to_le_bytes().as_slice()),  3593052489046108800);
        assert_eq!(                  rapidhash(0x10000000_00000001u128.to_le_bytes().as_slice()),  7365235785575411947);
        assert_eq!(         rapidhash(0x10000000_00000000_00000000u128.to_le_bytes().as_slice()),  5399386355486589714);
        assert_eq!(rapidhash(0x10000000_00000000_00000000_00000000u128.to_le_bytes().as_slice()), 13365378750111633005);
        assert_eq!(rapidhash(0xffffffff_ffffffff_ffffffff_ffffffffu128.to_le_bytes().as_slice()), 10466158564987642889);
    }
}
