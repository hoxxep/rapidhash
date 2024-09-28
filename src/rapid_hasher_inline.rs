use core::hash::Hasher;
use crate::rapid_const::{rapidhash_core, rapidhash_finish, rapidhash_seed, RAPID_SEED};

/// A [Hasher] trait compatible hasher that uses the [rapidhash](https://github.com/Nicoshev/rapidhash)
/// algorithm, and uses `#[inline(always)]` for all methods.
///
/// Using `#[inline(always)]` can deliver a large performance improvement when hashing complex
/// objects, but should be benchmarked for your specific use case. If you have HashMaps for many
/// different types this may come at the cost of some binary size increase.
///
/// See [crate::RapidHasher] for default non-forced inline methods.
///
/// See [RapidInlineHashBuilder] for usage with [std::collections::HashMap].
///
/// # Example
/// ```
/// use std::hash::Hasher;
/// use rapidhash::RapidInlineHasher;
///
/// let mut hasher = RapidInlineHasher::default();
/// hasher.write(b"hello world");
/// let hash = hasher.finish();
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidInlineHasher {
    seed: u64,
    a: u64,
    b: u64,
    size: u64,
}

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [RapidInlineHasher] algorithm.
///
/// This is an alias for [`std::hash::BuildHasherDefault<RapidHasher>`] with a static seed.
///
/// Note there that [crate::RapidRandomState] with the `rand` feature can be used instead for a
/// [std::hash::BuildHasher] that initialises with a random seed.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hasher;
/// use rapidhash::RapidInlineHashBuilder;
///
/// let mut map = HashMap::with_hasher(RapidInlineHashBuilder::default());
/// map.insert(42, "the answer");
/// ```
pub type RapidInlineHashBuilder = core::hash::BuildHasherDefault<RapidInlineHasher>;

/// A [std::collections::HashMap] type that uses the [RapidInlineHashBuilder] hasher.
///
/// # Example
/// ```
/// use rapidhash::RapidInlineHashMap;
/// let mut map = RapidInlineHashMap::default();
/// map.insert(42, "the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidInlineHashMap<K, V> = std::collections::HashMap<K, V, RapidInlineHashBuilder>;

/// A [std::collections::HashSet] type that uses the [RapidInlineHashBuilder] hasher.
///
/// # Example
/// ```
/// use rapidhash::RapidInlineHashSet;
/// let mut set = RapidInlineHashSet::default();
/// set.insert("the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidInlineHashSet<K> = std::collections::HashSet<K, RapidInlineHashBuilder>;

impl RapidInlineHasher {
    /// Default `RapidHasher` seed.
    pub const DEFAULT_SEED: u64 = RAPID_SEED;

    /// Create a new [RapidInlineHasher] with a custom seed.
    #[inline(always)]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            seed,
            a: 0,
            b: 0,
            size: 0,
        }
    }

    /// Create a new [RapidInlineHasher] using the default seed.
    #[inline(always)]
    #[must_use]
    pub const fn default_const() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }

    /// Const equivalent to [Hasher::write], and marked as `#[inline(always)]`.
    ///
    /// This can deliver a large performance improvement when the `bytes` length is known at compile
    /// time.
    #[inline(always)]
    #[must_use]
    pub const fn write_const(&self, bytes: &[u8]) -> Self {
        // FUTURE: wyhash processes the bytes as u64::MAX chunks in case chunk.len() > usize.
        // we use this static assert to ensure that usize is not larger than u64 for now.
        const _: () = assert!(
            usize::MAX as u128 <= u64::MAX as u128,
            "usize is wider than u64. Please raise a github issue to support this."
        );

        let mut this = *self;
        this.size += bytes.len() as u64;
        this.seed = rapidhash_seed(this.seed, this.size);
        let (a, b, seed) = rapidhash_core(this.a, this.b, this.seed, bytes);
        this.a = a;
        this.b = b;
        this.seed = seed;
        this
    }

    /// Const equivalent to [Hasher::finish], and marked as `#[inline(always)]`.
    #[inline(always)]
    #[must_use]
    pub const fn finish_const(&self) -> u64 {
        rapidhash_finish(self.a, self.b, self.size)
    }
}

impl Default for RapidInlineHasher {
    /// Create a new [RapidInlineHasher] with the default seed.
    #[inline(always)]
    fn default() -> Self {
        Self::new(RAPID_SEED)
    }
}

/// This implementation implements methods for all integer types as the compiler will (hopefully...)
/// inline and heavily optimize the rapidhash_core for each. Where the bytes length is known the
/// compiler can make significant optimisations and saves us writing them out by hand.
impl Hasher for RapidInlineHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.finish_const()
    }

    /// Write a byte slice to the hasher, marked as `#[inline(always)]`.
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        *self = self.write_const(bytes);
    }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        *self = self.write_const(&i.to_ne_bytes());

        // NOTE: in case of compiler regression, it should compile to:
        // self.size += size_of::<u64>() as u64;
        // self.seed ^= rapid_mix(self.seed ^ RAPID_SECRET[0], RAPID_SECRET[1]) ^ self.size;
        // self.a ^= i.rotate_right(32) ^ RAPID_SECRET[1];
        // self.b ^= i ^ self.seed;
        // rapid_mum(&mut self.a, &mut self.b);
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_i8(&mut self, i: i8) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_i16(&mut self, i: i16) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_i128(&mut self, i: i128) {
        *self = self.write_const(&i.to_ne_bytes());
    }

    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        *self = self.write_const(&i.to_ne_bytes());
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
            let mut hasher = RapidInlineHasher::default();
            hasher.write(int.to_ne_bytes().as_slice());
            let a = hasher.finish();

            assert_eq!(int.to_ne_bytes().as_slice().len(), 8);

            let mut hasher = RapidInlineHasher::default();
            hasher.write_u64(int);
            let b = hasher.finish();

            assert_eq!(a, b, "Mismatching hash for u64 with input {int}");
        }
    }
}
