use std::hash::Hasher;
use crate::rapid_const::{rapid_mix, rapid_mum, read_u32, read_u64, RAPID_SECRET};

/// A hybrid fxhash and [rapidhash] hasher.
///
/// Intended to deliver the best of both worlds for small and large inputs. Great for hashing
/// structs of mixed types.
///
/// This uses fxhash for small inputs 16 bytes or fewer, and rapidhash for larger inputs.
pub struct FxRapidHasher {
    hash: u64,
}

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [FxRapidHasher] algorithm.
///
/// This is a hybrid of the [fxhash](https://crates.io/crates/fxhash) and [rapidhash] algorithms.
///
/// This does not do any random seed initialization and is not HashDoS resistant.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hasher;
/// use rapidhash::FxRapidHashBuilder;
///
/// let mut map = HashMap::with_hasher(FxRapidHashBuilder::default());
/// map.insert(42, "the answer");
pub type FxRapidHashBuilder = std::hash::BuildHasherDefault<FxRapidHasher>;

/// A [std::collections::HashMap] type that uses the [FxRapidHashBuilder].
///
/// # Example
/// ```
/// use rapidhash::FxRapidHashMap;
/// let mut map = FxRapidHashMap::default();
/// map.insert(42, "the answer");
///
/// // with capacity
/// let mut map = FxRapidHashMap::with_capacity_and_hasher(10, Default::default());
/// map.insert(42, "the answer");
/// ```
pub type FxRapidHashMap<K, V> = std::collections::HashMap<K, V, FxRapidHashBuilder>;

/// A [std::collections::HashSet] type that uses the [FxRapidHashBuilder].
///
/// # Example
/// ```
/// use rapidhash::FxRapidHashSet;
/// let mut set = FxRapidHashSet::default();
/// set.insert("the answer");
///
/// // with capacity
/// let mut set = FxRapidHashSet::with_capacity_and_hasher(10, Default::default());
/// set.insert("the answer");
/// ```
pub type FxRapidHashSet<K> = std::collections::HashSet<K, FxRapidHashBuilder>;

/// Helper function to hash a single word, as part of [fxrapidhash].
#[inline(always)]
fn hash_word(hash: &mut u64, word: u64) {
    *hash = (hash.rotate_left(5) ^ word).wrapping_mul(0x517cc1b727220a95);
}

/// Hybrid fxhash and rapidhash function.
///
/// This method borrows code from [fxhash](https://github.com/cbreeden/fxhash/tree/master) which was
/// released under the MIT OR Apache 2.0 Licenses by @cbreeden.
///
/// This function seems to be _so_ sensitive to small changes that cause the compiler to do
/// something weird and half the performance of the thing. Setting explicit profiles in Cargo.toml
/// seems to help.
#[inline(always)]
pub fn fxrapidhash(mut bytes: &[u8], mut hash: u64) -> u64 {
    // On M1 Max, 17-22 is still faster using fxhash, but 19 and 23 are very slow. We could use
    // something like `(bytes.len() | 4) <= 22` instead, but it's very dataset-dependent. Our
    // benchmarks that test a single length like it, but when the condition is centered near the
    // median of a mixed-length dataset the branch predictor isn't happy.
    if bytes.len() <= 16 {
        while bytes.len() >= 8 {
            let n = read_u64(bytes, 0);
            hash_word(&mut hash, n);
            bytes = bytes.split_at(8).1;
        }

        if bytes.len() >= 4 {
            let n = read_u32(bytes, 0);
            hash_word(&mut hash, n as u64);
            bytes = bytes.split_at(4).1;
        }

        for byte in bytes {
            hash_word(&mut hash, *byte as u64);
        }

        hash
    } else {
        rapidhash_cold(bytes, hash)
    }
}

/// Rapidhash function only for `data.len() > 16` bytes.
///
/// We keep this method cold so that the hot path for small inputs is faster, while the function
/// call and single if statement are fairly insignificant for larger inputs.
///
/// Inlining this method seems to cause the fxrapidhash function to be slower overall for small
/// inputs.
#[cold]
#[inline(never)]
const fn rapidhash_cold(data: &[u8], mut seed: u64) -> u64 {
    seed ^= rapid_mix(seed ^ RAPID_SECRET[0], RAPID_SECRET[1]) ^ data.len() as u64;

    let mut slice = data;

    // most CPUs appear to benefit from this unrolled loop
    let mut see1 = seed;
    let mut see2 = seed;
    while slice.len() >= 96 {
        seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
        see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
        see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
        seed = rapid_mix(read_u64(slice , 48) ^ RAPID_SECRET[0], read_u64(slice, 56) ^ seed);
        see1 = rapid_mix(read_u64(slice, 64) ^ RAPID_SECRET[1], read_u64(slice, 72) ^ see1);
        see2 = rapid_mix(read_u64(slice, 80) ^ RAPID_SECRET[2], read_u64(slice, 88) ^ see2);
        let (_, split) = slice.split_at(96);
        slice = split;
    }
    if slice.len() >= 48 {
        seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
        see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
        see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
        let (_, split) = slice.split_at(48);
        slice = split;
    }
    seed ^= see1 ^ see2;

    if slice.len() > 16 {
        seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[2], read_u64(slice, 8) ^ seed ^ RAPID_SECRET[1]);
        if slice.len() > 32 {
            seed = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[2], read_u64(slice, 24) ^ seed);
        }
    }

    let a = read_u64(data, data.len() - 16) ^ RAPID_SECRET[1];
    let b = read_u64(data, data.len() - 8) ^ seed;

    let (a, b) = rapid_mum(a, b);
    rapid_mix(a ^ RAPID_SECRET[0] ^ data.len() as u64, b ^ RAPID_SECRET[1])
}

impl FxRapidHasher {
    /// Create a new [FxRapidHasher] with a custom seed.
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { hash: seed }
    }
}

impl Default for FxRapidHasher {
    #[inline]
    fn default() -> Self {
        Self { hash: 0 }
    }
}

impl Hasher for FxRapidHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = fxrapidhash(bytes, self.hash);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        hash_word(&mut self.hash, i);
        // self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.hash = fxrapidhash(&i.to_ne_bytes(), self.hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A quick test to check hash outputs differ.
    ///
    /// As can be seen from the tests, fxhash is very weak for some inputs, a single bit flip
    /// often only changes a single bit in the output!
    ///
    /// These tests are not deterministic, but should fail with low probability.
    #[test]
    fn flip_bit_trial() {
        use rand::Rng;

        let mut flips = std::vec![];

        for len in 1..=256 {
            let mut data = std::vec![0; len];
            rand::thread_rng().fill(&mut data[..]);

            let hash = fxrapidhash(&data, 0);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;
                    let new_hash = fxrapidhash(&data, 0);
                    assert_ne!(hash, new_hash, "Flipping bit {} did not change hash", byte);
                    let xor = hash ^ new_hash;
                    let flipped = xor.count_ones() as u64;
                    assert!(xor.count_ones() >= 1, "Flipping bit {byte}:{bit} changed only {flipped} bits");

                    flips.push(flipped);
                }
            }
        }

        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
    }
}
