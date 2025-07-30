use core::hash::{BuildHasher, Hash, Hasher};
use crate::inner::RapidHasher;
use crate::inner::seed::rapidhash_seed;

/// A [std::collections::hash_map::RandomState] compatible hasher that initializes the [RapidHasher]
/// algorithm with a random seed.
///
/// Note this is not sufficient to prevent HashDoS attacks. The rapidhash algorithm is not proven to
/// be resistant, and the seed used is not wide enough.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use std::hash::Hasher;
///
/// use rapidhash::quality::RandomState;
///
/// let mut map = HashMap::with_hasher(RandomState::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandomState<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> {
    seed: u64,
    secrets: &'static [u64; 7],
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// Create a new random state with a random seed.
    ///
    /// With the `rand` feature enabled, this will use [rand::random] to initialise the seed.
    ///
    /// Without `rand` but with the `std` feature enabled, this will use [crate::rapidrng_time] to
    /// initialise the seed.
    #[inline]
    #[cfg(target_has_atomic = "ptr")]
    pub fn new() -> Self {
        Self {
            seed: super::seeding::seed::get_seed(),
            secrets: super::seeding::secrets::get_secrets(),
        }
    }

    /// Create a state with a specific seed.
    #[inline]
    #[cfg(target_has_atomic = "ptr")]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: rapidhash_seed(seed),
            secrets: super::seeding::secrets::get_secrets(),
        }
    }

    /// Create a state with a specific seed and secrets.
    #[inline]
    pub const fn with_seed_and_static_secrets(seed: u64, secrets: &'static [u64; 7]) -> Self {
        Self {
            seed: rapidhash_seed(seed),
            secrets,
        }
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Default for RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool>  BuildHasher for RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    type Hasher = RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new_precomputed_seed(self.seed, self.secrets)
    }

    #[inline]
    fn hash_one<T: Hash>(&self, x: T) -> u64
    where
        Self: Sized,
        Self::Hasher: Hasher,
    {
        let mut hasher = self.build_hasher();
        x.hash(&mut hasher);  // <-- trying hard to inline this
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use core::hash::BuildHasher;

    type RandomState = super::RandomState<false, true, false, false>;

    #[test]
    fn test_random_state() {
        let state1 = RandomState::new();
        let state2 = RandomState::new();

        let finish1a = state1.hash_one(b"hello");
        let finish1b = state1.hash_one(b"hello");
        let finish2a = state2.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);
    }

    #[test]
    fn test_static_secrets() {
        static SECRETS: [u64; 7] = [
            0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7
        ];

        let state1a = RandomState::with_seed_and_static_secrets(0, &SECRETS);
        let state1b = RandomState::with_seed_and_static_secrets(0, &SECRETS);
        let state2a = RandomState::with_seed_and_static_secrets(1, &SECRETS);

        let finish1a = state1a.hash_one(b"hello");
        let finish1b = state1b.hash_one(b"hello");
        let finish2a = state2a.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);
    }
}
