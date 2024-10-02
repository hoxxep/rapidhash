# rapidhash - rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

- **High quality**, the fastest hash passing all tests in the SMHasher and SMHasher3 benchmark. Collision-based study showed a collision probability lower than wyhash and close to ideal.
- **Very fast**, the fastest passing hash in SMHasher3. Significant throughput improvement over wyhash. Fastest memory-safe hash. Fastest platform-independent hash. Fastest const hash.
- **Platform independent**, works on all platforms, no dependency on machine-specific vectorized or cryptographic hardware instructions. Optimised for both AMD64 and AArch64.
- **Memory safe**, when the `unsafe` feature is disabled (default). This implementation has also been fuzz-tested with `cargo fuzz`.
- **No dependencies and no-std compatible** when disabling the `std` feature.
- **Official successor to wyhash**, with improved speed, quality, and compatibility.
- **Inline variants** that use `#[inline(always)]` on `RapidInlineHash` and `RapidInlineHashBuilder` to force compiler optimisations on specific input types (can double the hash performance depending on the hashed type).
- **Run-time and compile-time hashing** as the hash implementation is fully `const`.
- **Idiomatic** `std::hash::Hasher` compatible hasher for `HashMap` and `HashSet` usage.
- **Non-cryptographic** hash function.

## Usage
### Hashing
```rust
use std::hash::Hasher;
use rapidhash::{rapidhash, RapidInlineHasher, RapidHasher};

// direct const usage
assert_eq!(rapidhash(b"hello world"), 17498481775468162579);

// a std::hash::Hasher compatible hasher
let mut hasher = RapidInlineHasher::default();
hasher.write(b"hello world");
assert_eq!(hasher.finish(), 17498481775468162579);

// a non-inline hasher for when you don't want to force inlining,
// such as when being careful with WASM binary size.\
let mut hasher = RapidInlineHasher::default();
hasher.write(b"hello world");
assert_eq!(hasher.finish(), 17498481775468162579);

// a const API similar to std::hash::Hasher
const HASH: u64 = RapidInlineHasher::default_const()
    .write_const(b"hello world")
    .finish_const();
assert_eq!(HASH, 17498481775468162579);
```

### Helper Types
```rust
// also includes HashSet equivalents
use rapidhash::{RapidHashMap, RapidInlineHashMap};

// std HashMap with the RapidHashBuilder hasher.
let mut map = RapidHashMap::default();
map.insert("hello", "world");

// a hash map type using the RapidInlineHashBuilder to force the compiler to
// inline the hash function for further optimisations (can be over 30% faster).
let mut map = RapidInlineHashMap::default();
map.insert("hello", "world");
```

## Features

- `default`: `std`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables `RapidRandomState`, a `BuildHasher` that randomly initializes the seed. Includes the `rand` crate dependency.
- `rng`: Enables `RapidRng`, a fast, non-cryptographic random number generator based on rapidhash. Includes the `rand_core` crate dependency.
- `unsafe`: Uses unsafe pointer arithmetic to skip some unnecessary bounds checks for a small 3-4% performance improvement.

## TODO
This repo is an active work in progress.

- [ ] Benchmark against the C++ implementation via FFI.
- [ ] Benchmark on x86_64 server platforms.
- [ ] Add rapidhash protected variant.
- [ ] Publish to crates.io. (Currently in the process of requesting the rapidhash crate name.)

## How to choose your hash function

Hash functions are not a one-size fits all. Benchmark your use case to find the best hash function for your needs, but here are some general guidelines on choosing a hash function:

- `default`: Use the std lib hasher when hashing is not in the critical path, or if you need strong HashDoS resistance.
- `rapidhash`: You are hashing complex objects or byte streams, need compile-time hashing, or a performant high-quality hash. Benchmark the `RapidInline` variants if you need the utmost performance.
- `fxhash`: You are hashing integers, or structs of only integers, and the lower quality hash doesn't affect your use case.
- `gxhash`: You are hashing long byte streams on platforms with the necessary instruction sets and only care about throughput. You don't need memory safety, HashDoS resistance, or platform independence (for example, gxhash doesn't currently compile on Github Actions workflows).

## Benchmarks

Initial benchmarks on M1 Max (aarch64) for various input sizes.

### Hashing Benchmarks
There are two types of benchmarks over the different algorithms to cover various forms of compiler optimisation that Rust can achieve:
- `str_len`: hashing bytes (a string) of the given length, where the length is not known at compile time.
- `u64`: hashing a u64, 8 bytes of known size, where the compiler can optimise the path.

Note on wyhash: hashing throughput doesn't translate to hashmap insertion throughput, see the hashmap insertion benchmarks below.

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash.svg)

### HashMap Insertion Benchmarks

Hash speed and throughput can be a poor measure in isolation, as it doesn't take into account hash quality. More hash collisions can cause slower hashmap insertion, and so hashmap insertion benchmarks can be a better measure of hash performance. As always, benchmark your use case.

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_insert.svg)

## Versioning
The minimum supported Rust version (MSRV) is 1.77.0.

The rapidhash crate follows the following versioning scheme:
- Major for breaking changes, such as hash output changes, breaking API changes, MSRV version bumps. When the RNG code is stabilised, major version bumps to `rand_core` will also trigger a major version bump of rapidhash due to the re-exported trait implementations.
- Minor for significant API additions/deprecations.
- Patch for bug fixes and performance improvements.

## License and Acknowledgements
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to [Nicolas De Carli](https://github.com/Nicoshev) for the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [BSD 2-Clause license](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).

With thanks to [Justin Bradford](https://github.com/jabr) for letting us use the rapidhash crate name 🍻
