use core::hash::BuildHasher;
use crate::inner::RapidHasher;

/// A `std::hash::BuildHasher` trait compatible hasher that uses the [`RapidHasher`] algorithm
/// with the default fixed seed and secrets.
///
/// This is not recommended unless you need determinism between program runs, but please note that
/// stable hash outputs are not guaranteed between either rapidhash versions, compiler versions, or
/// different platforms.
///
/// # Not HashDoS Resistant
/// These secrets are **NOT HashDoS resistant**, as they use the default rapidhash secrets. Instead,
/// consider using [`crate::inner::GlobalState`] for HashDoS resistant hashing with secrets that are
/// static for the lifetime of the program.
///
/// # Portable Hashing
/// FixedState is not suitable for portable hashing. Please use [`crate::v3::rapidhash_v3`] and
/// similar methods instead.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FixedState<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> {}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> FixedState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// Create a new fixed state with a fixed seed and secrets.
    ///
    /// # Not HashDoS Resistant
    /// These secrets are **NOT HashDoS resistant**, as they use the default rapidhash secrets. Instead,
    /// consider using [`crate::inner::GlobalState`] for HashDoS resistant hashing with secrets that
    /// are static for the lifetime of the program.
    ///
    /// # Portable Hashing
    /// FixedState is not suitable for portable hashing. Please use [`crate::v3::rapidhash_v3`] and
    /// similar methods instead.
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Default for FixedState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool>  BuildHasher for FixedState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    type Hasher = RapidHasher<'static, AVALANCHE, SPONGE, COMPACT, PROTECTED>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new_precomputed_seed(
            crate::inner::seed::rapidhash_seed(0),
            &crate::inner::seed::DEFAULT_SECRETS,
        )
    }
}

#[cfg(test)]
mod tests {
    use core::hash::BuildHasher;

    type FixedState = super::FixedState<false, true, false, false>;

    #[test]
    fn test_global_state() {
        assert_eq!(core::mem::size_of::<FixedState>(), 0);

        let state1 = FixedState::new();
        let state2 = FixedState::new();

        let finish1a = state1.hash_one(b"hello");
        let finish1b = state1.hash_one(b"hello");
        let finish2a = state2.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_eq!(finish1a, finish2a);
    }
}
