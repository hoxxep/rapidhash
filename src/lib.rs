#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

#[deny(missing_docs)]
#[deny(unused_must_use)]

mod rapid_const;
mod rapid_hasher;
#[cfg(any(feature = "rng", docsrs))]
mod rng;
#[cfg(any(feature = "rand", docsrs))]
mod random_state;

#[doc(inline)]
pub use crate::rapid_hasher::*;

use crate::rapid_const::{rapidhash_raw, RAPID_SEED};

#[doc(inline)]
#[cfg(any(feature = "rand", docsrs))]
pub use crate::random_state::*;
#[doc(inline)]
#[cfg(any(feature = "rng", docsrs))]
pub use crate::rng::*;

/// Rapidhash a single byte stream, matching the C++ implementation.
#[inline]
pub const fn rapidhash(data: &[u8]) -> u64 {
    rapidhash_raw(data, RAPID_SEED)
}

/// Rapidhash a single byte stream, matching the C++ implementation, with a custom seed.
#[inline]
pub const fn rapidhash_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_raw(data, seed)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::hash::{Hash, Hasher};
    use std::collections::BTreeSet;
    use rand::Rng;
    use rand::rngs::OsRng;
    use super::*;

    #[derive(Hash)]
    struct Object {
        bytes: std::vec::Vec<u8>,
    }

    /// Check the [rapidhash] oneshot function is equivalent to [RapidHasher]
    #[test]
    fn hasher_equivalent_to_oneshot() {
        let hash = rapidhash(b"hello world");
        assert_ne!(hash, 0);
        assert_eq!(hash, 17498481775468162579);

        let mut hasher = RapidHasher::default();
        hasher.write(b"hello world");
        assert_eq!(hasher.finish(), 17498481775468162579);

        let hash = rapidhash(b"hello world!");
        assert_eq!(hash, 12238759925102402976);
    }

    /// `#[derive(Hash)]` writes a length prefix first, check understanding.
    #[test]
    fn derive_hash_works() {
        let object = Object { bytes: b"hello world".to_vec() };
        let mut hasher = RapidHasher::default();
        object.hash(&mut hasher);
        assert_eq!(hasher.finish(), 3415994554582211120);

        let mut hasher = RapidHasher::default();
        hasher.write_usize(b"hello world".len());
        hasher.write(b"hello world");
        assert_eq!(hasher.finish(), 3415994554582211120);
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

    /// Hardcoded hash values that are known to be correct.
    #[test]
    fn hashes_to_expected_values() {
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

    /// Ensure that changing a single bit flips at least 10 bits in the resulting hash, and on
    /// average flips half of the bits.
    ///
    /// These tests are not deterministic, but should fail with a very low probability.
    #[test]
    fn flip_bit_trial() {
        use rand::Rng;

        let mut flips = std::vec![];

        for len in 1..=256 {
            let mut data = std::vec![0; len];
            rand::thread_rng().fill(&mut data[..]);

            let hash = rapidhash(&data);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;
                    let new_hash = rapidhash(&data);
                    assert_ne!(hash, new_hash, "Flipping bit {} did not change hash", byte);
                    let xor = hash ^ new_hash;
                    let flipped = xor.count_ones() as u64;
                    assert!(xor.count_ones() >= 10, "Flipping bit {byte}:{bit} changed only {flipped} bits");

                    flips.push(flipped);
                }
            }
        }

        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
    }

    /// Helper method for [flip_bit_trial_streaming]. Hashes a byte stream in u8 chunks.
    fn streaming_hash(data: &[u8]) -> u64 {
        let mut hasher = RapidHasher::default();
        for byte in data {
            hasher.write_u8(*byte);
        }
        hasher.finish()
    }

    /// The same as [flip_bit_trial], but against our streaming implementation, to ensure that
    /// reusing the `a`, `b`, and `seed` state is not causing glaringly obvious issues.
    ///
    /// This test is not a substitute for SMHasher or similar.
    ///
    /// These tests are not deterministic, but should fail with a very low probability.
    #[test]
    fn flip_bit_trial_streaming() {
        use rand::Rng;

        let mut flips = std::vec![];

        for len in 1..=256 {
            let mut data = std::vec![0; len];
            rand::thread_rng().fill(&mut data[..]);

            let hash = streaming_hash(&data);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;

                    // check that the hash changed
                    let new_hash = streaming_hash(&data);
                    assert_ne!(hash, new_hash, "Flipping bit {} did not change hash", byte);

                    // track how many bits were flipped
                    let xor = hash ^ new_hash;
                    let flipped = xor.count_ones() as u64;
                    assert!(xor.count_ones() >= 10, "Flipping bit {byte}:{bit} changed only {flipped} bits");
                    flips.push(flipped);
                }
            }
        }

        // check that on average half of the bits were flipped
        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
    }
}
