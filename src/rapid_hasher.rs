use core::hash::Hasher;
use crate::rapid_const::{RAPID_SEED};
use crate::RapidInlineHasher;

/// A [Hasher] trait compatible hasher that uses the [rapidhash](https://github.com/Nicoshev/rapidhash) algorithm.
///
/// See [RapidInlineHasher] for an `#[inline(always)]` version of this hasher, which can deliver
/// speed improvements of around 30% when hashing complex objects.
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
pub struct RapidHasher(RapidInlineHasher);

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// This is an alias for [`std::hash::BuildHasherDefault<RapidHasher>`] with a static seed.
///
/// See [RapidInlineHasher] for an `#[inline(always)]` version of this hasher, which can deliver
/// speed improvements of around 30% when hashing complex objects.
///
/// See [crate::RapidRandomState] with the `rand` feature can be used instead for a
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
/// See [crate::RapidInlineHashMap] for an `#[inline(always)]` version of this type, which can deliver
/// speed improvements of around 30% when hashing complex objects.
///
/// # Example
/// ```
/// use rapidhash::RapidHashMap;
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashMap<K, V> = std::collections::HashMap<K, V, RapidHashBuilder>;

/// A [std::collections::HashSet] type that uses the [RapidHashBuilder] hasher.
///
/// See [crate::RapidInlineHashSet] for an `#[inline(always)]` version of this type, which can
/// deliver speed improvements of around 30% when hashing complex objects.
///
/// # Example
/// ```
/// use rapidhash::RapidHashMap;
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashSet<K> = std::collections::HashSet<K, RapidHashBuilder>;

impl RapidHasher {
    /// Default `RapidHasher` seed.
    pub const DEFAULT_SEED: u64 = RAPID_SEED;

    /// Create a new [RapidHasher] with a custom seed.
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self(RapidInlineHasher::new(seed))
    }

    /// Create a new [RapidHasher] using the default seed.
    #[inline]
    #[must_use]
    pub const fn default_const() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }

    /// Const equivalent to [Hasher::write].
    ///
    /// # Example
    /// ```rust
    /// use rapidhash::RapidHasher;
    ///
    /// let hasher = RapidHasher::default_const();
    /// let hash = hasher
    ///     .write_const(b"some bytes")
    ///     .write_const(b"and some more bytes")
    ///     .finish_const();
    /// ```
    #[inline]
    #[must_use]
    pub const fn write_const(&self, bytes: &[u8]) -> Self {
        Self(self.0.write_const(bytes))
    }

    /// Const equivalent to [Hasher::finish].
    #[inline]
    #[must_use]
    pub const fn finish_const(&self) -> u64 {
        self.0.finish_const()
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
        self.0.finish_const()
    }

    /// Write a byte slice to the hasher.
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0.write_u8(i)
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0.write_u16(i)
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0.write_u32(i)
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0.write_u64(i)
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0.write_u128(i)
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0.write_usize(i)
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.0.write_i8(i)
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.0.write_i16(i)
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.0.write_i32(i)
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.0.write_i64(i)
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.0.write_i128(i)
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.0.write_isize(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher_write_u64() {
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
