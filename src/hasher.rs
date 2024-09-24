use core::hash::Hasher;
use crate::rapid::{rapid_mix, rapidhash_core, rapidhash_finish, RAPID_SECRET, RAPID_SEED};

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
#[derive(Copy, Clone)]
pub struct RapidHasher {
    seed: u64,
    a: u64,
    b: u64,
    size: u64,
}

/// A [BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// With the `rand` feature:
/// - enabled: this is an alias for [RapidRandomState].
/// - disabled: this is an alias for [BuildHasherDefault<RapidHasher>] with a static seed.
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
#[cfg(not(feature = "rand"))]
pub type RapidHashBuilder = core::hash::BuildHasherDefault<RapidHasher>;

/// A [BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// With the `rand` feature:
/// - enabled: this is an alias for [RapidRandomState].
/// - disabled: this is an alias for [BuildHasherDefault<RapidHasher>] with a static seed.
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
#[cfg(feature = "rand")]
pub type RapidHashBuilder = crate::random::RapidRandomState;

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
}

impl Default for RapidHasher {
    /// Create a new [RapidHasher] with the default seed.
    #[inline]
    fn default() -> Self {
        Self::new(RAPID_SEED)
    }
}

impl Hasher for RapidHasher {
    #[inline]
    fn finish(&self) -> u64 {
        rapidhash_finish(self.a, self.b, self.size)
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // FUTURE: wyhash processes the bytes as u64::MAX chunks in case chunk.len() > usize.
        const _: () = assert!(usize::MAX as u128 <= u64::MAX as u128, "usize is larger than u64. Please raise a github issue to support this.");

        self.seed ^= rapid_mix(self.seed ^ RAPID_SECRET[0], RAPID_SECRET[1]);
        let chunk = bytes;
        self.seed ^= chunk.len() as u64;
        let (a, b, seed) = rapidhash_core(self.a, self.b, self.seed, chunk);
        self.a = a;
        self.b = b;
        self.seed = seed;
        self.size += chunk.len() as u64;
    }
}
