# Changelog

## 4.3.0 (20260208)

### Additions
- Implemented `Debug` on `RandomState`, `GlobalState`, and `SeedableState`. (@anp in #78 and @hoxxep in #79)

## 4.2.2 (20260205)

### Fixes
- Include LICENSE files in the published crate. (@anp in https://github.com/hoxxep/rapidhash/pull/77)

## 4.2.1 (20260111)

### Fixes
- Updated `v3::rapidhash_v3_file` documentation to better describe the streaming interface.

## 4.2.0 (20251220)

### Additions
- Added `GlobalState` which initializes a `RapidHasher` with a global seed and secrets that are randomized once on application start.

## 4.1.1 (20251021)

### Fixes
- Fixed docsrs build flags, replacing `doc_auto_cfg` with `doc_cfg`. (https://github.com/hoxxep/rapidhash/pull/55)
- Fixed broken docsrs links. (https://github.com/hoxxep/rapidhash/pull/57)

## 4.1.0 (20250912)

### Additions
- Added `SeedableState::new` and `SeedableState::custom` to create hasher builders with user-defined seeds and secrets.

### Deprecations
- `SeedableState::with_seed` has been deprecated in favour of `SeedableState::custom` for better naming and improved warnings in the documentation.

## 4.0.0 (20250901)

https://github.com/hoxxep/rapidhash/pull/43

### Breaking changes
- **`rapidhash::v3` micro/nano output change:** input lengths 5-7 were mismatching the intended C++ V3 output. The C++ rapidhash V3 has been [yanked and re-released as V3](https://github.com/Nicoshev/rapidhash/issues/33) to fix the bug, and this rust implementation will follow. This changes the hash outputs for `rapidhash_v3_micro_inline` and `rapidhash_v3_nano_inline` for inputs of size 5, 6, and 7 bytes.
- **`RapidBuildHasher` renamed and refactored** to `SeedableState`.
- **`RapidHasher<'s>` new lifetime parameter** added to support user-defined secrets via `SeedableState`.
- **`RapidHashMap` and `RapidHashSet` moved to crate top level** for convenience. The top level uses the `fast::` variants, and the `quality::` and `inner::` hashmaps have been removed. They can still be built manually using `inner::RandomState` if required. The `fast::` collection variants have been deprecated to be removed in a future major release.

### Additions
- **`nightly` feature** which improves str hashing performance by omitting the `0xFF` suffix write and adds likely/unlikely hints.
- **`SeedableState`**: a hasher builder which can be seeded with fixed or user-defined secrets. This replaces `RapidBuildHasher`, but still defaults to random seeds. It is slightly slower than `RandomState`.

### Performance improvements
- **Bounds check elision**: Improved `RapidHasher` by eliding extra bounds checks in some cases by using `assert_unchecked`.
- **Likely/unlikely hints**: Added stable likely/unlikely hints in various places to ensure small inputs are favoured.

### MSRV
- **MSRV reduced to 1.71.0** from 1.77.0 by removing const usage of `first_chunk`.

## 3.1.0 (20250809)

### Performance improvements
- Improved `RapidHasher` small string hashing performance by 1.5-15% depending on the benchmark, by reducing the small string hashing code size and allowing the compiler to inline more. Performance was also improved on big-endian platforms by reading native-endian bytes. The portable hashers (`rapidhash::v3` etc. modules) are unaffected by this change. [#37](https://github.com/hoxxep/rapidhash/pull/37)

### Fixes
- Fixed compilation on targets without atomic pointers. [#38](https://github.com/hoxxep/rapidhash/issues/38), [#39](https://github.com/hoxxep/rapidhash/pull/39)

## 3.0.0 (20250730)

Big performance improvements, and potentially rust's fastest general-purpose hasher!

### Breaking changes
- Replaced `FNV` with a `SPONGE` configuration with `RapidHasher` to improve integer and tuple hashing performance.
- `RandomState` removed the `with_seed` and `with_seed_and_static_secrets` methods to reduce the struct size. Please raise a GitHub issue if you need a `SeededState`-style hash builder for the in-memory `RapidHasher`.
- Added a `RapidSecrets` type for all versions to generate seeds and secrets for HashDoS-resistant hashing. This makes generating unique seeds/secrets easier for persistent hashing use cases.
  - `rapidhash::v*` portable hashing functions now all take a `&RapidSecrets` argument as a seed.
  - For full compatibility with the old integer seeds, instantiate `RapidSecrets::seed_cpp(u64)` with your integer seed which will continue to use the default secrets.
  - For minimal DoS resistance, use `RapidSecrets::seed(u64)` to generate a new seed and secrets.
- `RapidHasher` removed the old `write_const` and `finish_const` methods as they were unlikely to be used and may cause confusion.
- `rapidhash_v*_inner` methods have a new `AVALANCHE` const argument to control whether they should avalanche the output. Setting this to `true` will match the default hash output and C++ implementation for the highest quality hashing. Turning off avalanching will reduce the hash quality, but improve performance, especially for small inputs.
- Fixed the `rapidhash::v1` `V1_BUG` argument to actually match the original `v1.x.x` crate behaviour, which changes the hash output. The if statement was fundamentally wrong in the `v2.x.x` crate and failing to hash some bytes, apologies. This now has a proper test to prevent future regressions.

### Additions
- `rapidhash::rng::rapidrng_fast_non_portable`: a slightly faster, lower-quality RNG that also has optimisations for u32 platforms without wide-arithmetic support. Excellent for generating fixtures in our WASM benchmarks.

### Deprecations
- The V1 and V2 `rapidhash_file` methods have been deprecated, with a note to use `rapidhash::v3` instead. This is because they aren't streaming-compatible hashing algorithms which may be misleading if someone has not read the documentation in detail. They will continue to be included for the foreseeable future as they provide the CLI functionality.

### Performance improvements
- `RapidHasher` significantly improved performance hashing integers, tuples, and integer types using the new `SPONGE` configuration in both fast and quality modes.
- `RapidHasher` now uses a non-portable mixing function for an improvement on platforms with slow wide arithmetic, such as wasm32.
- `rapidhash::v3` has a healthy performance improvement for mid-size input lengths by skipping the 112+ length setup/teardown.

## 2.0.2 (20250723)

- Fix docs.rs crashing with a broken README link.

## 2.0.1 (20250723)

- Minor documentation improvements.

## 2.0.0 (20250723)

**Rapidhash algorithm changes** have pushed us towards a refactor of this crate. Compatibility with rapidhash V2.0, V2.1, V2.2, and V3 are now all supported under `rapidhash::v3` and `rapidhash::v2` modules. These expose `rapidhash_v3` and similar methods to avoid version confusion. Each version produces different hash outputs.

- **Breaking:** `RapidHasher` in-memory hasher overhaul:
  - `RapidHasher` deviates from the main rapidhash algorithm to improve performance hashing rust objects while maintaining similar hash quality. Performance should be significantly improved over the v1 crate.
  - `RapidHasher` may change the underlying hash between minor versions. The rust `Hasher` trait is not to be used for portable hashing, and we will follow this mantra to allow easily improving hashing performance. Portable hashing should be done though the `rapidhash::v3::rapidhash_v3(bytes: &[u8])` and equivalent functions.
  - `RapidHasher`, `RandomState`, `RapidHashMap`, and `RapidHashSet` now move behind the following three modules:
    - `rapidhash::fast` when hashing speed is the priority. This sacrifices some hash quality for speed, uses FNV when hashing integer types, and skips the final avalanche mixing step.
    - `rapidhash::quality` when hash quality is the ultimate priority. This closely resembles the rapidhash algorithm and hash quality.
    - `rapidhash::inner` when you want to configure the settings for `AVALANCHE`, `FNV`, `COMPACT`, and `PROTECTED` modes as necessary.
- **Breaking:** `rapidhash` portable hashing function moved and renamed, with different hash output:
  - Fixed the rapidhash V1 algorithm for 48 and 144 length inputs, where it would previously mismatch with the C implementation.
    - If you need the old broken rapidhash V1 hash output, `rapidhash::v1::rapidhash_v1_inline` can accept a compile time argument `V1_BUG=true`, which will reproduce the old hash output from the 1.x crate versions.
  - Moved and renamed `rapidhash::rapidhash()` to `rapidhash::v1::rapidhash_v1()` to allow us to include other rapidhash versions in the same naming convention.
- **Breaking:** Random number generation has been moved behind the `rng` module, but otherwise works the same.
- **Breaking:** `RapidRandomState` has been renamed to `RandomState`, and moved into `fast`, `quality`, and `inner` modules.
- **Breaking:** Removed `rapid_mix` and `rapid_mum` from the public API for cleanliness. These are now internal functions.
- **Breaking:** Removed the `RapidInline*` variants in favour of making the default `Rapid` types inline by default. If this is a problem for your use case, please raise a GitHub issue, thanks!
- **Breaking:** Removed the deprecated `RapidHashBuilder` type, it has been replaced with `RapidBuildHasher` to match the rust naming convention.
- New: Added support for rapidhash V2.0, V2.1, V2.2, and V3 algorithms.
- New: rapidhash CLI now supports streaming properly with the V3 algorithm.
- Fix: Full tests and verification against the C implementations for all versions.
- Perf: Extensive benchmarking and optimisation, see more: https://github.com/hoxxep/rapidhash/issues/20

## 1.4.0 (20250219)

- Updated `rand` and `rand-core` to 0.9. [#18](https://github.com/hoxxep/rapidhash/pull/18)
- Fixed issue where using feature `unsafe` and without `std` would fail to compile. [#15](https://github.com/hoxxep/rapidhash/issues/15) and [#17](https://github.com/hoxxep/rapidhash/pull/17)

## 1.3.0 (20241208)

- Added `rapidhash_file` for streaming file hashing. [#10](https://github.com/hoxxep/rapidhash/pull/10)
- Added file streaming and `--help` to the CLI. [#10](https://github.com/hoxxep/rapidhash/pull/10)

## 1.2.0 (20241204)

- Added rapidhash CLI via `cargo install rapidhash`.
- Docs typo fix.

## 1.1.0 (20241003)

- Deprecated `RapidHashBuilder`.
- Added `RapidBuildHasher` to replace `RapidHashBuilder`.

## 1.0.0 (20241002)

Ownership kindly transferred by Justin Bradford to [Liam Gray](https://github.com/hoxxep) and this repository.

- **Breaking:** Removed the `hash` function that only hashes on `u128` types.
- Added `rapidhash` and `rapidhash_seeded` functions to hash byte streams.
- Added `RapidHasher` and `RapidHasherInline` for hashing via a `std::hash::Hasher` compatible interface.
- Added `RapidHashMap`, `RapidInlineHashMap`, `RapidHashSet`, and `RapidInlineHashSet` helper types.
- Added `RapidHashBuilder` and `RapidInlineHashBuilder` for `std::hash::BuildHasher` implementing types compatible with `HashMap` and `HashSet`.
- Added `RapidRandomState` for random seed initialization.
- Added `RapidRng`, `rapidrng_fast`, and `rapidrng_time` for random number generation inspired by the [wyhash crate](https://docs.rs/wyhash/latest/wyhash/) but based on `rapid_mix`.
- Added `std`, `rand`, `rng`, and `unsafe` features.
- Extensive benchmarking and optimisation.

## 0.1.0

Initial release by [Justin Bradford](https://github.com/jabr) supporting rapidhash on `u128` inputs.

- Added `hash` for rapidhashing `u128` types.
