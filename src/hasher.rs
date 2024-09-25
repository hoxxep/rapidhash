use core::hash::Hasher;
use crate::rapid::{rapidhash_core, rapidhash_finish, rapidhash_seed, RAPID_SEED};

/// A [Hasher] trait compatible hasher that uses the [rapidhash](https://github.com/Nicoshev/rapidhash) algorithm.
///
/// See [RapidHashBuilder] for usage with [std::collections::HashMap].
///
/// # Example
/// ```
/// use std::hash::Hasher;
/// use rapidhash::RapidHasher;
///
/// let mut hasher = RapidHasher::default();
/// hasher.write(b"hello world");
/// let hash = hasher.finish();
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidHasher {
    seed: u64,
    a: u64,
    b: u64,
    size: u64,
}

/// A [BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// This is an alias for [BuildHasherDefault<RapidHasher>] with a static seed.
///
/// Note there that [crate::RapidRandomState] with the `rand` feature can be used instead for a
/// [std::hash::BuildHasher] that initialises with a random seed.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hasher;
/// use rapidhash::RapidHashBuilder;
///
/// let mut map = HashMap::with_hasher(RapidHashBuilder::default());
/// map.insert(42, "the answer");
/// ```
pub type RapidHashBuilder = core::hash::BuildHasherDefault<RapidHasher>;

/// A [std::collections::HashMap] type that uses the [RapidHashBuilder] hasher.
///
/// # Example
/// ```
/// use rapidhash::RapidHashMap;
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
/// ```
#[cfg(feature = "std")]
pub type RapidHashMap<K, V> = std::collections::HashMap<K, V, RapidHashBuilder>;

/// A [std::collections::HashSet] type that uses the [RapidHashBuilder] hasher.
///
/// # Example
/// ```
/// use rapidhash::RapidHashMap;
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
/// ```
#[cfg(feature = "std")]
pub type RapidHashSet<K> = std::collections::HashSet<K, RapidHashBuilder>;

impl RapidHasher {
    /// Create a new [RapidHasher] with a custom seed.
    ///
    /// It is recommended to use [RapidHasher::default] instead.
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            a: 0,
            b: 0,
            size: 0,
        }
    }

    /// Equivalent to [Hasher::write] but always inlines the function.
    ///
    /// This can deliver a large performance improvement when the `bytes` length is known.
    #[inline(always)]
    pub fn write_always_inline(&mut self, bytes: &[u8]) {
        // FUTURE: wyhash processes the bytes as u64::MAX chunks in case chunk.len() > usize.
        // we use this static assert to ensure that usize is not larger than u64 for now.
        const _: () = assert!(
            usize::MAX as u128 <= u64::MAX as u128,
            "usize is wider than u64. Please raise a github issue to support this."
        );

        self.size += bytes.len() as u64;
        self.seed = rapidhash_seed(self.seed, self.size);
        let (a, b, seed) = rapidhash_core(self.a, self.b, self.seed, bytes);
        self.a = a;
        self.b = b;
        self.seed = seed;
    }
}

impl Default for RapidHasher {
    /// Create a new [RapidHasher] with the default seed.
    #[inline]
    fn default() -> Self {
        Self::new(RAPID_SEED)
    }
}

/// This implementation implements methods for all integer types as the compiler will (hopefully...)
/// inline and heavily optimize the rapidhash_core for each. Where the bytes length is known the
/// compiler can make significant optimisations and saves us writing them out by hand.
impl Hasher for RapidHasher {
    #[inline]
    fn finish(&self) -> u64 {
        rapidhash_finish(self.a, self.b, self.size)
    }

    /// Write a byte slice to the hasher.
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.write_always_inline(bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write_always_inline(&i.to_ne_bytes());

        // NOTE: in case of compiler regression, it should compile to:
        // self.size += size_of::<u64>() as u64;
        // self.seed ^= rapid_mix(self.seed ^ RAPID_SECRET[0], RAPID_SECRET[1]) ^ self.size;
        // self.a ^= i.rotate_right(32) ^ RAPID_SECRET[1];
        // self.b ^= i ^ self.seed;
        // rapid_mum(&mut self.a, &mut self.b);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write_always_inline(&i.to_ne_bytes());
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_always_inline(&i.to_ne_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher_write_u64() {
        assert_eq!((8 & 24) >> (8 >> 3), 4);

        let ints = [
            1234u64,
            0,
            1,
            u64::MAX,
            u64::MAX - 2385962040453523
        ];

        for int in ints {
            let mut hasher = RapidHasher::default();
            hasher.write(int.to_ne_bytes().as_slice());
            let a = hasher.finish();

            assert_eq!(int.to_ne_bytes().as_slice().len(), 8);

            let mut hasher = RapidHasher::default();
            hasher.write_u64(int);
            let b = hasher.finish();

            assert_eq!(a, b, "Mismatching hash for u64 with input {int}");
        }
    }
}
