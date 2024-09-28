use std::hash::{BuildHasher, RandomState};
use crate::RapidHasher;

/// An [std::collections::hash_map::RandomState] compatible hasher that uses the [RapidHasher]
/// algorithm.
///
/// This randomly initialises the [RapidHasher] seed for a small improvement against hash collision
/// attacks.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use std::hash::Hasher;
/// use rapidhash::RapidRandomState;
///
/// let mut map = HashMap::with_hasher(RapidRandomState::default());
/// map.insert(42, "the answer");
/// ```
#[cfg(any(feature = "rand", docsrs))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidRandomState {
    seed: u64,
}

impl RapidRandomState {
    pub fn new() -> Self {
        // TODO: check whether we should randomly init (a, b) too.
        let seed: u64 = rand::random();

        Self {
            seed,
        }
    }
}

impl Default for RapidRandomState {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RapidRandomState {
    type Hasher = RapidHasher;

    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new(self.seed)
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{BuildHasher, Hasher, RandomState};

    #[test]
    fn test_random_state() {
        // the same state should produce the equivalent hashes
        let state1 = RandomState::new();
        let mut hash1a = state1.build_hasher();
        let mut hash1b = state1.build_hasher();

        // different state should produce different hashes
        let state2 = RandomState::new();
        let mut hash2a = state2.build_hasher();

        hash1a.write(b"hello");
        hash1b.write(b"hello");
        hash2a.write(b"hello");

        let finish1a = hash1a.finish();
        let finish1b = hash1b.finish();
        let finish2a = hash2a.finish();

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);
    }
}
