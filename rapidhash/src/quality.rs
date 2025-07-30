//! In-memory hashing: RapidHasher with a focus on hash quality.
//!
//! This is a specific instantiation of the [rapidhash::inner] module with the following settings:
//! - `AVALANCHE` is enabled.
//! - `SPONGE` is enabled.
//! - `COMPACT` is disabled, unless building for WASM targets.
//! - `PROTECTED` is disabled.

const AVALANCHE: bool = true;
const SPONGE: bool = true;
const COMPACT: bool = cfg!(target_family = "wasm");
const PROTECTED: bool = false;

use crate::inner;

/// A [Hasher] inspired by [rapidhash::v3::rapidhash_v3] with a focus on speed and throughput.
///
/// This is an alias for [inner::RapidHasher] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::RapidHasher] for a lower quality but faster hash output where desirable.
pub type RapidHasher = inner::RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A [std::hash::BuildHasher] inspired by [rapidhash::v3::rapidhash_v3] with a focus on hash quality.
///
/// This is an alias for [inner::RapidBuildHasher] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::RapidHasher] for a lower quality but faster hash output where desirable.
pub type RapidBuildHasher = inner::RapidBuildHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A rapidhash equivalent to [std::hash::RandomState] that uses a random seed and secrets for
/// minimal DoS resistance.
pub type RandomState = inner::RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A [std::collections::HashMap] that uses the [RapidHasher] hash.
///
/// This is an alias for [inner::RapidHashMap] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::RapidHashMap] where lower hash collision resistance is acceptable.
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashMap<K, V> = inner::RapidHashMap<K, V, AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A [std::collections::HashSet] that uses the [RapidHasher] hash.
///
/// This is an alias for [inner::RapidHashSet] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::RapidHashSet] where lower hash collision resistance is acceptable.
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashSet<K> = inner::RapidHashSet<K, AVALANCHE, SPONGE, COMPACT, PROTECTED>;

#[cfg(any(feature = "std", docsrs))]
pub use inner::HashMapExt;
#[cfg(any(feature = "std", docsrs))]
pub use inner::HashSetExt;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_new() {
        let mut map = RapidHashMap::new();
        map.insert("key", "value");
    }

    #[test]
    fn test_hashset_new() {
        let mut set = RapidHashSet::new();
        set.insert("value");
    }
}
