use core::hash::{BuildHasher, Hash, Hasher};
use super::DEFAULT_RAPID_SECRETS;
use super::mix::rapid_mix_np;
use super::rapid_const::rapidhash_core;
use super::seed::rapidhash_seed;

/// A [Hasher] trait compatible hasher that uses the [rapidhash](https://github.com/Nicoshev/rapidhash)
/// algorithm, and uses `#[inline(always)]` for all methods.
///
/// Using `#[inline(always)]` can deliver a large performance improvement when hashing complex
/// objects, but should be benchmarked for your specific use case. If you have HashMaps for many
/// different types this may come at the cost of some binary size increase.
///
/// See [crate::RapidHasher] for default non-forced inline methods.
///
/// See [RapidHashBuilder] for usage with [std::collections::HashMap].
///
/// # Example
/// ```
/// use std::hash::Hasher;
///
/// use rapidhash::quality::RapidHasher;
///
/// let mut hasher = RapidHasher::default();
/// hasher.write(b"hello world");
/// let hash = hasher.finish();
/// ```
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RapidHasher<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> {
    seed: u64,
    secrets: &'static [u64; 7],  // FUTURE: non-static secrets?
    sponge: u128,
    sponge_len: u8,
}

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// This is an alias for [`std::hash::BuildHasherDefault<RapidHasher>`] with a static seed.
///
/// Note there that [crate::RapidRandomState] with can be used instead for a
/// [std::hash::BuildHasher] that initialises with a random seed.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hasher;
///
/// use rapidhash::quality::RapidBuildHasher;
///
/// let mut map = HashMap::with_hasher(RapidBuildHasher::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidBuildHasher<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> {
    seed: u64,
    secrets: &'static [u64; 7],
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> RapidBuildHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// New rapid inline build hasher, and pre-compute the seed.
    #[inline]
    pub const fn new(mut seed: u64) -> Self {
        seed = rapidhash_seed(seed);
        Self { seed, secrets: &DEFAULT_RAPID_SECRETS.secrets }
    }
}

// Explicitly implement to inline always the hasher.
impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> BuildHasher for RapidBuildHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    type Hasher = RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::new_precomputed_seed(self.seed, self.secrets)
    }

    /// The aim of the game here is twofold:
    /// - let the compiler inline as much as possible
    /// - ensure the compiler prioritises inlining `x.hash()`, which has the biggest boost to
    ///   performance by allowing it to optimise out most of the sponge logic
    ///
    /// This is evident in the realworld benchmarks — only run one benchmark with inline everything
    /// and hashing is up to 2x faster. Play your cards wrong with variables in the wrong order or
    /// too many instructions and if x.hash() isn't inlined it can be 5x slower in a bunch of
    /// benchmarks... Frustrating voodoo to be up against! An alternative is to remove the sponge
    /// and hash numbers in a more optimal way.
    ///
    /// Ultimately `write_num` and `finish()` are more important to be inlined than the
    /// `write(bytes)` as they can optimise away the sponge flushing/if logic. Write bytes is
    /// simply incurring a single function call, unless the bytes are of compile-time known length,
    /// in which case there are large gains again.
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

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Default for RapidBuildHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    #[inline]
    fn default() -> Self {
        Self::new(RapidHasher::<AVALANCHE, SPONGE, COMPACT, PROTECTED>::DEFAULT_SEED)
    }
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// Default `RapidHasher` seed.
    pub const DEFAULT_SEED: u64 = super::seed::DEFAULT_SEED;

    /// Create a new [RapidHasher] with a custom seed.
    ///
    /// See instead [src::quality::RandomState::new] or [src::fast::RandomState::new] for a random
    /// seed and random secret initialisation, for minimal DoS resistance.
    #[inline(always)]
    #[must_use]
    pub const fn new(mut seed: u64) -> Self {
        // do most of the rapidhash_seed initialisation here to avoid doing it on each int
        seed = rapidhash_seed(seed);
        Self::new_precomputed_seed(seed, &DEFAULT_RAPID_SECRETS.secrets)
    }

    #[inline(always)]
    #[must_use]
    pub(super) const fn new_precomputed_seed(seed: u64, secrets: &'static [u64; 7]) -> Self {
        Self {
            seed,
            secrets,
            sponge: 0,
            sponge_len: 0,
        }
    }

    /// Create a new [RapidHasher] using the default seed and secrets.
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
        this.seed = rapidhash_core::<AVALANCHE, COMPACT, PROTECTED>(this.seed, this.secrets, bytes);
        this
    }

    /// This function needs to be as small as possible to have as high a chance of being inlined as
    /// possible. So we use good-old SPONGE where the entropy won't be lost, and fold for 64bit inputs.
    ///
    /// N = number of _bits_ in the integer type.
    #[inline(always)]
    #[must_use]
    const fn write_num<const N: u8>(&self, bytes: u64) -> Self {
        let mut this = *self;

        if SPONGE {
            if this.sponge_len + N > 128 {
                // sponge is full, so we need to flush it
                let a = this.sponge as u64;
                let b = (this.sponge >> 64) as u64;
                this.seed = rapid_mix_np::<PROTECTED>(a ^ this.secrets[1], b ^ this.seed);
                this.sponge = bytes as u128;
                this.sponge_len = N;
            } else {
                // OR the bytes into the sponge
                this.sponge |= (bytes as u128) << this.sponge_len;
                this.sponge_len += N;
            }
        } else {
            // slower but high-quality rapidhash
            this.seed = rapid_mix_np::<PROTECTED>(bytes ^ this.secrets[1], bytes ^ this.seed);
        }

        this
    }

    /// Straightforward fold for 128bit aligned inputs.
    #[inline(always)]
    #[must_use]
    const fn write_128(&self, bytes: u128) -> Self {
        let mut this = *self;
        let a = bytes as u64;
        let b = (bytes >> 64) as u64;
        this.seed = rapid_mix_np::<PROTECTED>(a ^ this.secrets[1], b ^ this.seed);

        if SPONGE && AVALANCHE {
            // if the sponge is being used, u128's won't otherwise be avalanched
            this.seed = rapid_mix_np::<PROTECTED>(this.seed, this.secrets[0]);
        }

        this
    }

    /// Const equivalent to [Hasher::finish], and marked as `#[inline(always)]`.
    #[inline(always)]
    #[must_use]
    pub const fn finish_const(&self) -> u64 {
        let mut seed = self.seed;

        if SPONGE && self.sponge_len > 0 {
            let a = self.sponge as u64;
            let b = (self.sponge >> 64) as u64;
            seed = rapid_mix_np::<PROTECTED>(a ^ seed, b ^ self.secrets[1]);

            if AVALANCHE {
                // any integer that's added to the sponge will cause the sponge_len to never be 0,
                // so avalanching inside this if is sufficient to avalanche all sponged inputs.
                seed = rapid_mix_np::<PROTECTED>(seed, self.secrets[0]);
            }
        }

        if !SPONGE && AVALANCHE {
            // if not using a sponge, we only avalanche integers at the very end
            seed = rapid_mix_np::<PROTECTED>(seed, self.secrets[0]);
        }

        seed
    }
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Default for RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// Create a new [RapidHasher] with the default seed.
    ///
    /// See [crate::RapidRandomState] for a [std::hash::BuildHasher] that initialises with a random
    /// seed.
    #[inline(always)]
    fn default() -> Self {
        Self::new(super::seed::DEFAULT_SEED)
    }
}

/// This implementation implements methods for all integer types as the compiler will (hopefully...)
/// inline and heavily optimize the rapidhash_core for each. Where the bytes length is known the
/// compiler can make significant optimisations and saves us writing them out by hand.
impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Hasher for RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
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
        *self = self.write_num::<8>(i.into());
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        *self = self.write_num::<16>(i.into());
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        *self = self.write_num::<32>(i.into());
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        *self = self.write_num::<64>(i);
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        #[cfg(target_pointer_width = "16")] {
            *self = self.write_num::<16>(i as u64);
        }

        #[cfg(target_pointer_width = "32")] {
            *self = self.write_num::<32>(i as u64);
        }

        #[cfg(target_pointer_width = "64")] {
            *self = self.write_num::<64>(i as u64);
        }
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        *self = self.write_num::<64>(i as u64);
    }

    #[inline(always)]
    fn write_i8(&mut self, i: i8) {
        *self = self.write_num::<8>(i as u64);
    }

    #[inline(always)]
    fn write_i16(&mut self, i: i16) {
        *self = self.write_num::<16>(i as u64);
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        *self = self.write_num::<32>(i as u64);
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        *self = self.write_num::<64>(i as u64);
    }

    #[inline(always)]
    fn write_i128(&mut self, i: i128) {
        *self = self.write_128(i as u128);
    }

    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        #[cfg(target_pointer_width = "16")] {
            *self = self.write_num::<16>(i as u64);
        }

        #[cfg(target_pointer_width = "32")] {
            *self = self.write_num::<32>(i as u64);
        }

        #[cfg(target_pointer_width = "64")] {
            *self = self.write_num::<64>(i as u64);
        }
    }

    // #[cfg(feature = "nightly")]
    // #[inline(always)]
    // fn write_str(&mut self, s: &str) {
    //     self.write(s.as_bytes());
    // }
    //
    // #[cfg(feature = "nightly")]
    // #[inline(always)]
    // fn write_length_prefix(&mut self, len: usize) {
    //     self.seed = self.seed.wrapping_add(len);
    // }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    /// Test that writing a single u64 outputs the same as writing the equivalent bytes.
    ///
    /// Does not apply if the algorithm is using a sponge.
    #[ignore]
    #[test]
    fn test_hasher_write_u64() {
        assert_eq!((8 & 24) >> (8 >> 3), 4);

        let ints = [1234u64, 0, 1, u64::MAX, u64::MAX - 2385962040453523];

        for int in ints {
            let mut hasher = RapidHasher::<true, false>::default();
            hasher.write(int.to_le_bytes().as_slice());
            let a = hasher.finish();

            assert_eq!(int.to_le_bytes().as_slice().len(), 8);

            let mut hasher = RapidHasher::<true, false>::default();
            hasher.write_u64(int);
            let b = hasher.finish();

            assert_eq!(a, b, "Mismatching hash for u64 with input {int}");
        }
    }

    /// Check the number of collisions when writing numbers.
    #[test]
    #[ignore]
    #[cfg(feature = "std")]
    fn test_num_collisions() {
        let builder = RapidBuildHasher::<true, false>::default();
        let mut collisions = 0;
        let mut set = std::collections::HashSet::new();
        for i in 0..=u16::MAX {
            let hash_u16 = builder.hash_one(i) & 0xFFFFFF;
            if set.contains(&hash_u16) {
                collisions += 1;
            } else {
                set.insert(hash_u16);
            }

            // if i < 256 {
            //     let hash_u8 = builder.hash_one(i as u8) & 0xFFFF;
            //     if set.contains(&hash_u8) {
            //         collisions += 1;
            //     } else {
            //         set.insert(hash_u8);
            //     }
            // }
        }
        assert_eq!(collisions, 0, "Collisions found when hashing numbers");
    }
}
