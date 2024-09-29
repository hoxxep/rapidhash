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
use rapidhash::{rapidhash, RapidHasher};

// direct const usage
assert_eq!(rapidhash(b"hello world"), 17498481775468162579);

// a std::hash::Hasher compatible hasher
let mut hasher = RapidHasher::default();
hasher.write(b"hello world");
assert_eq!(hasher.finish(), 17498481775468162579);

// a const API similar to std::hash::Hasher
const HASH: u64 = RapidHasher::default_const()
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

- [ ] Make `RapidInline` the default?
- [ ] Benchmark against the C++ implementation via FFI.
- [ ] Benchmark graphs, and benchmark on x86_64 server platforms.
- [ ] Add rapidhash protected variant.
- [ ] A rapidhash-based random number generator (currently WIP).
- [ ] Publish to crates.io. (Currently in the process of requesting the rapidhash crate name.)

## When to use each hash function

Hash functions are not a one-size fits all. Benchmark your use case to find the best hash function for your needs, but here are some general guidelines on choosing a hash function:

- `default`: Use the std lib hasher when hashing is not in the critical path or you need strong HashDoS resistance.
- `rapidhash`: You are hashing complex objects or byte streams, need compile-time hashing, or a low-collision hash. Default to using the `RapidInline` variants unless binary size is a concern (such as for WASM targets).
- `fxhash`: You are hashing integers, or structs of only integers.
- `gxhash`: You are hashing long byte streams on platforms with the necessary instruction sets and only care about performance. You don't need memory safety, proven HashDoS resistance, or platform independence (for example, gxhash doesn't currently compile on Github Actions workflows).

## Benchmarks

Initial benchmarks on M1 Max (aarch64) for various input sizes. Proper benchmark graphs coming soon.

### Hashing Benchmarks
There are three types of benchmarks over the different algorithms to cover various forms of compiler optimisation that Rust can achieve:
- `str_len`: hashing bytes (a string) of the given length, where the length is not known at compile time.
- `u64`: hashing a u64, 8 bytes of known size, where the compiler can slightly optimise the path.
- `object`: hashing a struct of the following form via the `Hash` and `Hasher` traits.
```rust
#[derive(Hash)]
struct Object {
    a: u8,
    b: u64,
    s: String,
    v: Vec<u32>,
}
```

```text
hash/crate/input_bytes  time:   [5%        median    95%      ]

hash/rapidhash/str_2    time:   [2.3102 ns 2.3287 ns 2.3504 ns]
hash/rapidhash/str_8    time:   [2.1325 ns 2.1458 ns 2.1615 ns]
hash/rapidhash/str_16   time:   [2.1480 ns 2.1635 ns 2.1829 ns]
hash/rapidhash/str_64   time:   [3.3223 ns 3.3386 ns 3.3569 ns]
hash/rapidhash/str_100  time:   [4.4559 ns 4.4822 ns 4.5099 ns]
hash/rapidhash/str_177  time:   [6.6407 ns 6.6815 ns 6.7287 ns]
hash/rapidhash/str_256  time:   [8.1413 ns 8.2287 ns 8.3607 ns]
hash/rapidhash/str_1024 time:   [33.072 ns 33.138 ns 33.213 ns]
hash/rapidhash/str_4096 time:   [143.04 ns 143.57 ns 144.36 ns]
hash/rapidhash/u8       time:   [1.1161 ns 1.1491 ns 1.1760 ns]
hash/rapidhash/u16      time:   [1.4396 ns 1.4495 ns 1.4600 ns]
hash/rapidhash/u32      time:   [1.2767 ns 1.2921 ns 1.3077 ns]
hash/rapidhash/u64      time:   [1.2664 ns 1.2766 ns 1.2868 ns]
hash/rapidhash/u128     time:   [1.7484 ns 1.7803 ns 1.8119 ns]
hash/rapidhash/object   time:   [17.255 ns 17.344 ns 17.441 ns]
hash/rapidhash/object_inline
                        time:   [7.6355 ns 7.6616 ns 7.6857 ns]

hash/default/str_2      time:   [5.4913 ns 5.5070 ns 5.5248 ns]
hash/default/str_8      time:   [6.4975 ns 6.5571 ns 6.6593 ns]
hash/default/str_16     time:   [7.8830 ns 7.9036 ns 7.9265 ns]
hash/default/str_64     time:   [17.964 ns 18.035 ns 18.111 ns]
hash/default/str_256    time:   [72.786 ns 74.969 ns 78.077 ns]
hash/default/str_1024   time:   [290.57 ns 290.88 ns 291.23 ns]
hash/default/str_4096   time:   [1.1666 µs 1.1677 µs 1.1689 µs]
hash/default/u64        time:   [7.6266 ns 7.6741 ns 7.7260 ns]
hash/default/object     time:   [34.635 ns 34.727 ns 34.817 ns]

hash/fxhash/str_2       time:   [1.3102 ns 1.3368 ns 1.3679 ns]
hash/fxhash/str_8       time:   [982.94 ps 1.0003 ns 1.0192 ns]
hash/fxhash/str_16      time:   [1.3843 ns 1.4039 ns 1.4289 ns]
hash/fxhash/str_64      time:   [4.0900 ns 4.1193 ns 4.1546 ns]
hash/fxhash/str_256     time:   [20.390 ns 20.461 ns 20.534 ns]
hash/fxhash/str_1024    time:   [136.43 ns 136.66 ns 136.92 ns]
hash/fxhash/str_4096    time:   [730.49 ns 731.34 ns 732.28 ns]
hash/fxhash/u64         time:   [890.02 ps 909.37 ps 928.01 ps]
hash/fxhash/object      time:   [6.8636 ns 6.8953 ns 6.9276 ns]

hash/gxhash/str_2       time:   [2.5633 ns 2.6010 ns 2.6443 ns]
hash/gxhash/str_8       time:   [2.5602 ns 2.6449 ns 2.7706 ns]
hash/gxhash/str_16      time:   [2.5881 ns 2.6530 ns 2.7477 ns]
hash/gxhash/str_64      time:   [3.3580 ns 3.4055 ns 3.4567 ns]
hash/gxhash/str_256     time:   [7.6870 ns 7.8056 ns 7.9192 ns]
hash/gxhash/str_1024    time:   [17.547 ns 17.798 ns 18.047 ns]
hash/gxhash/str_4096    time:   [61.094 ns 64.937 ns 71.977 ns]
hash/gxhash/u64         time:   [1.0981 ns 1.1164 ns 1.1334 ns]
hash/gxhash/object      time:   [5.2970 ns 5.3257 ns 5.3563 ns]

hash/ahash/str_2        time:   [2.8815 ns 2.9023 ns 2.9241 ns]
hash/ahash/str_8        time:   [2.8560 ns 2.8748 ns 2.8988 ns]
hash/ahash/str_16       time:   [2.8021 ns 2.8300 ns 2.8641 ns]
hash/ahash/str_64       time:   [4.6048 ns 4.6278 ns 4.6548 ns]
hash/ahash/str_256      time:   [14.133 ns 14.201 ns 14.279 ns]
hash/ahash/str_1024     time:   [57.845 ns 57.977 ns 58.118 ns]
hash/ahash/str_4096     time:   [264.18 ns 265.25 ns 266.40 ns]
hash/ahash/u64          time:   [1.8231 ns 1.8495 ns 1.8749 ns]
hash/ahash/object       time:   [6.1482 ns 6.1801 ns 6.2114 ns]

hash/t1ha/str_2         time:   [2.8340 ns 2.8494 ns 2.8673 ns]
hash/t1ha/str_8         time:   [2.8275 ns 2.8407 ns 2.8556 ns]
hash/t1ha/str_16        time:   [2.9214 ns 2.9432 ns 2.9685 ns]
hash/t1ha/str_64        time:   [5.8668 ns 5.8897 ns 5.9149 ns]
hash/t1ha/str_256       time:   [15.620 ns 15.784 ns 16.031 ns]
hash/t1ha/str_1024      time:   [67.880 ns 68.065 ns 68.290 ns]
hash/t1ha/str_4096      time:   [282.31 ns 283.83 ns 286.43 ns]
hash/t1ha/u64           time:   [3.3213 ns 3.3469 ns 3.3708 ns]
hash/t1ha/object        time:   [16.714 ns 16.749 ns 16.783 ns]

hash/wyhash/str_2       time:   [2.8576 ns 2.8713 ns 2.8863 ns]
hash/wyhash/str_8       time:   [2.8569 ns 2.8819 ns 2.9102 ns]
hash/wyhash/str_16      time:   [3.2715 ns 3.2943 ns 3.3206 ns]
hash/wyhash/str_64      time:   [4.3635 ns 4.3918 ns 4.4240 ns]
hash/wyhash/str_256     time:   [11.731 ns 11.801 ns 11.871 ns]
hash/wyhash/str_1024    time:   [42.997 ns 43.088 ns 43.187 ns]
hash/wyhash/str_4096    time:   [195.61 ns 199.12 ns 203.71 ns]
hash/wyhash/u64         time:   [1.1936 ns 1.2085 ns 1.2227 ns]
hash/wyhash/object      time:   [12.829 ns 12.983 ns 13.231 ns]

hash/xxhash/str_2       time:   [8.0915 ns 8.1123 ns 8.1360 ns]
hash/xxhash/str_8       time:   [7.2377 ns 7.2725 ns 7.3156 ns]
hash/xxhash/str_16      time:   [7.6382 ns 7.8046 ns 8.1398 ns]
hash/xxhash/str_64      time:   [9.2842 ns 9.3120 ns 9.3430 ns]
hash/xxhash/str_256     time:   [19.063 ns 19.143 ns 19.231 ns]
hash/xxhash/str_1024    time:   [39.883 ns 39.980 ns 40.091 ns]
hash/xxhash/str_4096    time:   [151.96 ns 152.33 ns 152.73 ns]
hash/xxhash/u64         time:   [8.6543 ns 8.6907 ns 8.7276 ns]
hash/xxhash/object      time:   [34.990 ns 35.099 ns 35.222 ns]

hash/metrohash/str_2    time:   [5.6428 ns 5.6633 ns 5.6859 ns]
hash/metrohash/str_8    time:   [5.9872 ns 6.1702 ns 6.5168 ns]
hash/metrohash/str_16   time:   [6.2684 ns 6.3092 ns 6.3596 ns]
hash/metrohash/str_64   time:   [8.8555 ns 8.8872 ns 8.9246 ns]
hash/metrohash/str_256  time:   [18.193 ns 18.246 ns 18.299 ns]
hash/metrohash/str_1024 time:   [57.506 ns 57.673 ns 57.894 ns]
hash/metrohash/str_4096 time:   [219.60 ns 219.98 ns 220.41 ns]
hash/metrohash/u64      time:   [5.8264 ns 5.8593 ns 5.8925 ns]
hash/metrohash/object   time:   [32.913 ns 32.967 ns 33.029 ns]

hash/seahash/str_2      time:   [5.1516 ns 5.1848 ns 5.2220 ns]
hash/seahash/str_8      time:   [5.3446 ns 5.3667 ns 5.3915 ns]
hash/seahash/str_16     time:   [5.6755 ns 5.6972 ns 5.7209 ns]
hash/seahash/str_64     time:   [8.7225 ns 8.7561 ns 8.7928 ns]
hash/seahash/str_256    time:   [27.643 ns 27.724 ns 27.806 ns]
hash/seahash/str_1024   time:   [116.30 ns 118.19 ns 121.17 ns]
hash/seahash/str_4096   time:   [472.41 ns 473.23 ns 474.11 ns]
hash/seahash/u64        time:   [6.4829 ns 6.5186 ns 6.5536 ns]
hash/seahash/object     time:   [47.885 ns 47.933 ns 47.983 ns]
```

### HashMap Insertion Benchmarks

Hash throughput speed is great and all, but hash quality also affects hashmap insertion speed. More hash collisions cause slower hashmap insertion, and so hashmap insertion benchmarks can be a better measure of hash performance. As always, benchmark your use case.

```text
map/crate/elems_min_max time:   [5%        median    95%      ]

map/rapidhash/1000_4_4  time:   [47.169 µs 47.258 µs 47.341 µs]
map/rapidhash/1000_4_64 time:   [63.136 µs 64.934 µs 67.438 µs]
map/rapidhash/100000_u64
                        time:   [1.6945 ms 1.6980 ms 1.7016 ms]
map/rapidhash/450000_words
                        time:   [34.284 ms 34.504 ms 34.762 ms]

map/rapidhash_inline/1000_4_4
                        time:   [35.670 µs 35.877 µs 36.216 µs]
map/rapidhash_inline/1000_4_64
                        time:   [60.759 µs 61.423 µs 62.458 µs]
map/rapidhash_inline/100000_u64
                        time:   [1.6957 ms 1.6993 ms 1.7030 ms]
map/rapidhash_inline/450000_words
                        time:   [29.486 ms 29.680 ms 29.907 ms]

map/default/1000_4_4    time:   [60.796 µs 61.633 µs 62.886 µs]
map/default/1000_4_64   time:   [95.880 µs 96.251 µs 96.828 µs]
map/default/100000_u64  time:   [4.2298 ms 4.2812 ms 4.3507 ms]
map/default/450000_words
                        time:   [47.657 ms 48.093 ms 48.670 ms]

map/fxhash/1000_4_4     time:   [32.732 µs 33.111 µs 33.825 µs]
map/fxhash/1000_4_64    time:   [70.490 µs 71.751 µs 74.139 µs]
map/fxhash/100000_u64   time:   [1.5145 ms 1.5183 ms 1.5224 ms]
map/fxhash/450000_words time:   [34.351 ms 34.565 ms 34.804 ms]

map/gxhash/1000_4_4     time:   [36.205 µs 37.533 µs 39.467 µs]
map/gxhash/1000_4_64    time:   [59.843 µs 60.174 µs 60.662 µs]
map/gxhash/100000_u64   time:   [1.8243 ms 1.8500 ms 1.8888 ms]
map/gxhash/450000_words time:   [27.190 ms 27.326 ms 27.481 ms]
```

## Versioning
The minimum supported Rust version (MSRV) is 1.77.0.

The rapidhash crate follows the following versioning scheme:
- Major for breaking changes, such as hash output changes, breaking API changes, MSRV version bumps. When the RNG code is stabilised, major version bumps to `rand_core` will also trigger a major version bump of rapidhash due to the re-exported trait implementations.
- Minor for significant API additions/deprecations.
- Patch for bug fixes and performance improvements.

## License
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [BSD 2-Clause license](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).
