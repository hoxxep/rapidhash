use std::cell::Cell;
use std::hash::BuildHasher;
use crate::{rapidrng_fast, RapidHasher};

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
/// use rapidhash::RapidRandomState;
///
/// let mut map = HashMap::with_hasher(RapidRandomState::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidRandomState {
    seed: u64,
}

impl RapidRandomState {
    /// Create a new random state with a random seed.
    ///
    /// With the `rand` feature enabled, this will use [rand::random] to initialise the seed.
    ///
    /// Without `rand` but with the `std` feature enabled, this will use [rapidrng_time] to
    /// initialise the seed.
    pub fn new() -> Self {
        #[cfg(feature = "rand")]
        thread_local! {
            static RANDOM_SEED: Cell<u64> = {
                Cell::new(rand::random())
            }
        }

        #[cfg(all(feature = "std", not(feature = "rand")))]
        thread_local! {
            static RANDOM_SEED: Cell<u64> = {
                let mut seed = crate::RAPID_SEED;
                Cell::new(crate::rapidrng_time(&mut seed))
            }
        }

        let mut seed = RANDOM_SEED.with(|cell| {
            let seed = cell.get();
            cell.set(seed.wrapping_add(1));
            seed
        });

        Self {
            seed: rapidrng_fast(&mut seed),
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
