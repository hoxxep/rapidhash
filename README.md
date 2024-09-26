# rapidhash - rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, which itself is the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

Extremely fast, high-quality, platform-independent, memory safe, no-std compatible, non-cryptographic hash function.

From the C++ implementation:
> Passes all tests in both SMHasher and SMHasher3, collision-based study showed a collision probability lower than wyhash and close to ideal.

## Usage

```rust
use std::hash::Hasher;
use rapidhash::{rapidhash, RapidHasher, RapidHashMap};

// a std::hash::Hasher compatible hasher
let mut hasher = RapidHasher::default();
hasher.write(b"hello world");
assert_eq!(hasher.finish(), 17498481775468162579);

// with the "std" feature, using the RapidHashMap helper type
let mut map = RapidHashMap::default();
map.insert("hello", "world");

// raw usage
assert_eq!(rapidhash(b"hello world"), 17498481775468162579);
```

## Features

- `default`: `std`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables the `rand` crate dependency (in no-std mode) and exports  `RapidRandomState` to randomly initialize the seed.
- `rng`: Enables `RapidRng`, a fast, non-cryptographic random number generator based on rapidhash.
- `unsafe`: Enables unsafe pointer arithmetic to skip some unnecessary bounds checks on small byte slice inputs (len <= 16), for a 3-4% performance improvement.

## TODO
This repo is an active work in progress.

- [x] Implement the basic `rapidhash` function.
- [x] Build a `RapidHasher` for the `std::hash::Hasher` trait.
- [x] Add more tests, benchmark comparisons, and further docs.
- [x] Review assembly output, and review code for small input sizes.
- [ ] Benchmark against the C++ implementation and confirm outputs match exactly.
- [ ] Benchmark graphs, and benchmark on x86_64 server platforms.
- [ ] Add rapidhash protected variant.
- [x] License the code under a permissive license.
- [ ] Publish to crates.io. (Currently in the process of requesting the rapidhash crate name.)

## Benchmarks
Initial benchmarks on M1 Max (aarch64) for various input sizes.

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

hash/rapidhash/str_2    time:   [2.3380 ns 2.3569 ns 2.3777 ns]
hash/rapidhash/str_8    time:   [2.6797 ns 2.7040 ns 2.7321 ns]
hash/rapidhash/str_16   time:   [2.6820 ns 2.6977 ns 2.7144 ns]
hash/rapidhash/str_64   time:   [3.5143 ns 3.5366 ns 3.5610 ns]
hash/rapidhash/str_256  time:   [8.4032 ns 8.4816 ns 8.5853 ns]
hash/rapidhash/str_1024 time:   [33.956 ns 34.275 ns 34.798 ns]
hash/rapidhash/str_4096 time:   [145.49 ns 145.78 ns 146.11 ns]
hash/rapidhash/u8       time:   [1.1307 ns 1.1646 ns 1.1917 ns]
hash/rapidhash/u16      time:   [1.4622 ns 1.4795 ns 1.4977 ns]
hash/rapidhash/u32      time:   [1.2956 ns 1.3173 ns 1.3398 ns]
hash/rapidhash/u64      time:   [1.2970 ns 1.3162 ns 1.3353 ns]
hash/rapidhash/u128     time:   [1.8301 ns 1.8758 ns 1.9172 ns]
hash/rapidhash/object   time:   [17.255 ns 17.344 ns 17.441 ns]

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

_Note: ahash is using the fallback algorithm on aarch64 targets and therefore these benchmarks. It's also not cross-platform nor stable, and the fallback algorithm is substantially weaker from the aes hashing algorithm._

## License
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [BSD 2-Clause license](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).
