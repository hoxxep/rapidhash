//! In-memory hashing: RapidHasher with full configurability via compile-time arguments.
//!
//! This module contains the Hasher, BuildHasher, HashMap, HashSet, and RandomState
//! implementations. It is recomended to use [rapidhash::fast] or [rapidhash::quality], but for the
//! advanced user, [rapidhash::inner] can be used directly to customise the compile time options to
//! modify the hash function.
//!
//! Each structure may have the compile time const generics:
//! - `AVALANCHE`: Whether to use a final avalanche mix step, required to pass SMHasher3. This
//!   option changes the hash output. Enabled on [rapidhash::quality], disabled on [rapidhash::fast].
//! - `SPONGE`: Allow RapidHasher to cache integers into a 128-bit buffer to perform a single
//!   folded multiply step on the entire buffer. If disabled, a mix step is performed on each
//!   individual integer. This changes the hash output when hashing integers. Enabled on both
//!   [rapidhash::quality] and [rapidhash::fast].
//! - `COMPACT`: Reduce the code size of the hasher by preventing manually unrolled loops. This does
//!   _not_ affect the hash output. Disabled on both [rapidhash::quality] and [rapidhash::fast].
//! - `PROTECTED`: When performing the folded multiply mix step, XOR the a and b back into their
//!   original values to make it harder for an attacker to generate collisions. This changes the
//!   hash ouput. Disabled on both [rapidhash::quality] and [rapidhash::fast].
//!
//! The `RapidHasher` struct is _inspired by_ rapidhash, but is not a direct port and will output
//! different hash values. It keeps the same hasher quality but uses various optimisations to
//! improve performance when used in the Rust Hasher trait.
//!
//! The output values of functions in the `inner` module are not guaranteed to be stable between
//! versions. Please use the `v1`, `v2`, or `v3` modules for stable output values between rapidhash
//! crate versions.


#[cfg(any(feature = "std", docsrs))]
mod collections;
mod rapid_const;
mod rapid_hasher;
mod state;
pub(crate) mod seeding;
mod mix;
mod seed;

#[doc(inline)]
pub use rapid_hasher::*;
#[doc(inline)]
#[cfg(any(feature = "std", docsrs))]
pub use collections::*;
#[doc(inline)]
pub use state::*;
#[doc(inline)]
use seed::*;

#[cfg(test)]
mod tests {
    extern crate std;

    use std::hash::{BuildHasher, Hash, Hasher};
    use std::collections::BTreeSet;
    use rand::Rng;
    use super::seed::{DEFAULT_RAPID_SECRETS, DEFAULT_SEED};
    use super::rapid_const::{rapidhash_rs, rapidhash_rs_seeded};

    type RapidHasher = super::RapidHasher<true, true, true>;
    type RapidBuildHasher = super::RapidBuildHasher<true, true, true>;

    #[derive(Hash)]
    struct Object {
        bytes: std::vec::Vec<u8>,
    }

    /// `#[derive(Hash)]` writes a length prefix first, check understanding.
    #[test]
    fn derive_hash_works() {
        let object = Object { bytes: b"hello world".to_vec() };
        let mut hasher = RapidHasher::default();
        object.hash(&mut hasher);
        assert_eq!(hasher.finish(), 1790036888308448300);

        let mut hasher = RapidHasher::default();
        hasher.write_usize(b"hello world".len());
        hasher.write(b"hello world");
        assert_eq!(hasher.finish(), 1790036888308448300);
    }

    /// Check RapidHasher is equivalent to the raw rapidhash for a single byte stream.
    ///
    /// Also check that the hash is unique for different byte streams.
    #[test]
    fn all_sizes() {
        let mut hashes = BTreeSet::new();

        for size in 0..=1024 {
            let mut data = std::vec![0; size];
            rand::rng().fill(data.as_mut_slice());

            let hash1 = rapidhash_rs(&data);
            let mut hasher = RapidHasher::default();
            hasher.write(&data);
            let hash2 = hasher.finish();

            assert_eq!(hash1, hash2, "Failed on size {}", size);
            assert!(!hashes.contains(&hash1), "Duplicate for size {}", size);

            hashes.insert(hash1);
        }
    }

    /// Ensure that changing a single bit flips at least 10 bits in the resulting hash, and on
    /// average flips half of the bits.
    ///
    /// These tests are not deterministic, but should fail with a very low probability.
    #[test]
    fn flip_bit_trial() {
        use rand::Rng;

        let mut flips = std::vec![];

        for len in 1..=512 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            let hash = rapidhash_rs(&data);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;
                    let new_hash = rapidhash_rs(&data);
                    assert_ne!(hash, new_hash, "Flipping byte {} bit {} did not change hash for input len {}", byte, bit, len);
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

        for len in 1..=300 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            let hash = streaming_hash(&data);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;

                    // check that the hash changed
                    let new_hash = streaming_hash(&data);
                    assert_ne!(hash, new_hash, "Flipping bit {byte}:{bit} for input len {len} did not change hash");

                    // track how many bits were flipped
                    let xor = hash ^ new_hash;
                    let flipped = xor.count_ones() as u64;
                    assert!(xor.count_ones() >= 10, "Flipping bit {byte}:{bit} for input len {len} changed only {flipped} bits");
                    flips.push(flipped);
                }
            }
        }

        // check that on average half of the bits were flipped
        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
    }

    /// Compare to the C rapidhash implementation to ensure we match perfectly.
    #[test]
    fn compare_to_c() {
        use rand::Rng;
        use rapidhash_c::rapidhashcc_rs;

        for len in 0..=512 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;

                    let rust_hash = rapidhash_rs_seeded(&data, &DEFAULT_RAPID_SECRETS);
                    let c_hash = rapidhashcc_rs(&data, DEFAULT_SEED);
                    assert_eq!(rust_hash, c_hash, "Mismatch with input {} byte {} bit {}", len, byte, bit);

                    let mut rust_hasher = RapidBuildHasher::default().build_hasher();
                    rust_hasher.write(&data);
                    let rust_hasher_hash = rust_hasher.finish();
                    assert_eq!(rust_hash, rust_hasher_hash, "Hasher mismatch with input {} byte {} bit {}", len, byte, bit);
                }
            }
        }
    }

    #[test]
    fn disambiguation_check() {
        use std::vec::Vec;

        let hasher = RapidBuildHasher::default();

        let a = [std::vec![1], std::vec![2, 3]];
        let b = [std::vec![1, 2], std::vec![3]];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = [std::vec![], std::vec![1]];
        let b = [std::vec![1],  std::vec![]];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a: [Vec<Vec<u64>>; 2] = [std::vec![], std::vec![std::vec![]]];
        let b: [Vec<Vec<u64>>; 2] = [std::vec![std::vec![]], std::vec![]];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = ["abc", "def"];
        let b = ["fed", "abc"];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = ["abc", "def"];
        let b = ["abcd", "ef"];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = [1u8, 2];
        let b = [2u8, 1];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = [1u16, 2];
        let b = [2u16, 1];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = [1u32, 2];
        let b = [2u32, 1];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = [1u64, 2];
        let b = [2u64, 1];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));

        let a = [1u128, 2];
        let b = [2u128, 1];
        assert_ne!(hasher.hash_one(a), hasher.hash_one(b));
    }
}
