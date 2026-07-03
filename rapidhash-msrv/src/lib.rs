//! This crate doubles as the home for rapidhash's struct-size checks.
//!
//! They live here, rather than in the `rapidhash` crate's own tests, for two reasons:
//! - This crate has minimal dependencies, so `cargo check --target <T>` cross-checks them cheaply
//!   (no target linker, emulator, or even libtest required) across a matrix of targets in CI. That
//!   is how we catch layout regressions on targets we can't natively run — e.g. s390x, where
//!   `u128` is 8-byte aligned rather than 16-byte, so the same struct is smaller than on
//!   x86-64/aarch64.
//! - As compile-time (`const`) assertions they would abort compilation if they ever tripped, so
//!   keeping them out of the published `rapidhash` crate ensures a bad bound can never break a
//!   downstream build — only this unpublished, path-only crate's compilation.
//!
//! The checks are deliberately *not* `#[cfg(test)]`: they fire on a plain `cargo check`, so they
//! also cover no-libtest targets like `thumbv6m-none-eabi` (no atomics, no std). Struct size is
//! endianness-independent, so the axes that matter are pointer width and `u128` alignment, plus the
//! target-specific `GlobalSecrets`/`COMPACT` instantiations — not byte order.

#![no_std]

/// Compile-time struct-size checks, evaluated on every compile of this crate.
///
/// Sizes are asserted as upper bounds because the `u128` sponge field's alignment — and therefore
/// the surrounding struct padding — is target-dependent: 16 bytes on x86-64/aarch64, 8 bytes on
/// s390x. `RandomState`/`GlobalState` use exact checks instead, to guard the invariant that their
/// `GlobalSecrets` field stays zero-sized (which is what keeps `HashMap<K, V, RandomState>` small).
mod size_checks {
    use core::mem::size_of;

    // Embeds the u128 sponge, so the size tracks u128 alignment.
    #[cfg(target_pointer_width = "64")]
    const _: () = assert!(size_of::<rapidhash::quality::RapidHasher<'static>>() <= 48);
    #[cfg(target_pointer_width = "32")]
    const _: () = assert!(size_of::<rapidhash::quality::RapidHasher<'static>>() <= 32);

    // Just a u64 seed and a reference; the same bound holds on 32- and 64-bit targets.
    const _: () = assert!(size_of::<rapidhash::fast::SeedableState<'static>>() <= 16);

    // A u64 seed plus a zero-sized GlobalSecrets, and the ZST alone, respectively.
    const _: () = assert!(size_of::<rapidhash::fast::RandomState>() == 8);
    const _: () = assert!(size_of::<rapidhash::fast::GlobalState>() == 0);

    #[cfg(feature = "std")]
    #[cfg(target_pointer_width = "64")]
    const _: () = assert!(size_of::<rapidhash::RapidHashMap<u64, u64>>() <= 40);
    #[cfg(feature = "std")]
    #[cfg(target_pointer_width = "32")]
    const _: () = assert!(size_of::<rapidhash::RapidHashMap<u64, u64>>() <= 24);
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    extern crate std;

    #[test]
    fn test_rapidhash() {
        assert_eq!(rapidhash::v3::rapidhash_v3(b"hello"), 3327445792987248966);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_hashmap() {
        use rapidhash::{RapidHashMap, HashMapExt};

        let mut map = RapidHashMap::new();
        map.insert("key", "value1");
        assert_eq!(map.get("key"), Some(&"value1"));
        assert_eq!(map.get("na"), None);
    }

    #[cfg(feature = "rng")]
    #[test]
    fn test_rng() {
        use rapidhash::rng::RapidRng;

        let mut rng = RapidRng::new(0);
        assert_ne!(rng.next(), rng.next());
    }
}
