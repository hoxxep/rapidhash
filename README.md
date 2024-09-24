# rapidhash - native rust implementation

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

- `default`: `std`, `rand`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables the `rand` crate dependency (in no-std mode) and exports  `RapidRandomState` to randomly initialize the seed.

## TODO
This repo is an active work in progress.

- [x] Implement the basic `rapidhash` function.
- [ ] Add rapidhash protected variant.
- [x] Evaluate building a `RapidHasher` for the `std::hash::Hasher` trait.
- [x] Add more tests, benchmark comparisons, and further docs.
- [ ] Review assembly output, and review code for small input sizes.
- [ ] Benchmark against the C++ implementation and confirm outputs match exactly.
- [ ] Benchmark graphs, and benchmark on x86_64 server platforms.
- [ ] License the code under a permissive license. Need to review whether this repo can be more permissive than the BSD 2-Clause the C++ crate is under.
- [ ] Publish to crates.io. (Currently in the process of requesting the rapidhash crate name, which currently is not this hash function.)

## Benchmarks
Initial benchmarks on M1 Max (aarch64) for various input sizes.

There are three types of benchmarks over the different algorithms to cover various forms of compiler optimisation that Rust can achieve:
- `str_len`: hashing bytes (a string) of the given length, where the length is not known at compile time.
- `u64`: hashing a u64, 8 bytes of known size, and so the compiler can heavily optimise the path (up to 10x faster!).
- `object`: hashing a struct of the following form via the `Hash` and `Hasher` traits.
```rust
#[derive(Hash)]
struct Object {
    a: u64,
    b: u64,
    s: String,
    v: Vec<u64>,
}
```

```text
hash/crate/input_bytes      time:   [5%        median    95%      ]

hash/rapidhash/str_2        time:   [11.392 ns 11.650 ns 11.949 ns]
hash/rapidhash/str_8        time:   [11.829 ns 12.129 ns 12.452 ns]
hash/rapidhash/str_16       time:   [11.723 ns 11.938 ns 12.188 ns]
hash/rapidhash/str_64       time:   [17.778 ns 18.011 ns 18.284 ns]
hash/rapidhash/str_256      time:   [37.018 ns 37.665 ns 38.396 ns]
hash/rapidhash/str_1024     time:   [56.768 ns 57.345 ns 58.120 ns]
hash/rapidhash/str_4096     time:   [169.04 ns 169.83 ns 171.00 ns]
hash/rapidhash/u64          time:   [1.2918 ns 1.3123 ns 1.3344 ns]
hash/rapidhash/object       time:   [32.689 ns 32.835 ns 32.981 ns]

hash/default/str_2          time:   [13.704 ns 13.877 ns 14.090 ns]
hash/default/str_8          time:   [14.922 ns 15.154 ns 15.439 ns]
hash/default/str_16         time:   [16.296 ns 16.651 ns 17.087 ns]
hash/default/str_64         time:   [28.827 ns 29.380 ns 30.115 ns]
hash/default/str_256        time:   [93.133 ns 94.249 ns 95.356 ns]
hash/default/str_1024       time:   [309.88 ns 310.65 ns 311.76 ns]
hash/default/str_4096       time:   [1.2017 µs 1.2039 µs 1.2064 µs]
hash/default/u64            time:   [3.6682 ns 3.6853 ns 3.7016 ns]
hash/default/object         time:   [55.050 ns 55.195 ns 55.350 ns]

hash/fxhash/str_2           time:   [9.8713 ns 10.082 ns 10.322 ns]
hash/fxhash/str_8           time:   [9.7616 ns 10.074 ns 10.600 ns]
hash/fxhash/str_16          time:   [10.309 ns 10.580 ns 10.894 ns]
hash/fxhash/str_64          time:   [18.521 ns 18.808 ns 19.179 ns]
hash/fxhash/str_256         time:   [45.988 ns 46.953 ns 47.941 ns]
hash/fxhash/str_1024        time:   [196.89 ns 197.83 ns 199.21 ns]
hash/fxhash/str_4096        time:   [794.81 ns 796.18 ns 797.85 ns]
hash/fxhash/u64             time:   [853.56 ps 873.47 ps 892.56 ps]
hash/fxhash/object          time:   [29.472 ns 29.624 ns 29.787 ns]

hash/t1ha/str_2             time:   [11.301 ns 11.461 ns 11.666 ns]
hash/t1ha/str_8             time:   [11.343 ns 11.609 ns 11.998 ns]
hash/t1ha/str_16            time:   [11.643 ns 12.101 ns 12.715 ns]
hash/t1ha/str_64            time:   [20.015 ns 20.604 ns 21.388 ns]
hash/t1ha/str_256           time:   [39.132 ns 40.343 ns 41.626 ns]
hash/t1ha/str_1024          time:   [89.958 ns 90.552 ns 91.433 ns]
hash/t1ha/str_4096          time:   [306.82 ns 309.06 ns 313.56 ns]
hash/t1ha/u64               time:   [3.6151 ns 3.6324 ns 3.6490 ns]
hash/t1ha/object            time:   [38.506 ns 38.668 ns 38.826 ns]

hash/wyhash/str_2           time:   [11.925 ns 12.133 ns 12.370 ns]
hash/wyhash/str_8           time:   [12.029 ns 12.222 ns 12.442 ns]
hash/wyhash/str_16          time:   [12.553 ns 12.841 ns 13.229 ns]
hash/wyhash/str_64          time:   [17.957 ns 18.649 ns 19.779 ns]
hash/wyhash/str_256         time:   [36.958 ns 37.833 ns 38.756 ns]
hash/wyhash/str_1024        time:   [68.948 ns 69.380 ns 69.910 ns]
hash/wyhash/str_4096        time:   [225.93 ns 226.83 ns 227.98 ns]
hash/wyhash/u64             time:   [1.1900 ns 1.2053 ns 1.2189 ns]
hash/wyhash/object          time:   [33.966 ns 34.136 ns 34.329 ns]

hash/xxhash/str_2           time:   [21.910 ns 22.102 ns 22.308 ns]
hash/xxhash/str_8           time:   [17.729 ns 17.942 ns 18.203 ns]
hash/xxhash/str_16          time:   [21.064 ns 21.536 ns 22.122 ns]
hash/xxhash/str_64          time:   [24.977 ns 25.267 ns 25.607 ns]
hash/xxhash/str_256         time:   [45.365 ns 46.052 ns 46.707 ns]
hash/xxhash/str_1024        time:   [67.169 ns 67.834 ns 68.717 ns]
hash/xxhash/str_4096        time:   [175.89 ns 176.34 ns 176.86 ns]
hash/xxhash/u64             time:   [8.6568 ns 8.7893 ns 8.9743 ns]
hash/xxhash/object          time:   [55.192 ns 55.361 ns 55.532 ns]

hash/metrohash/str_2        time:   [14.307 ns 14.479 ns 14.678 ns]
hash/metrohash/str_8        time:   [14.299 ns 14.533 ns 14.821 ns]
hash/metrohash/str_16       time:   [14.714 ns 15.052 ns 15.500 ns]
hash/metrohash/str_64       time:   [22.415 ns 22.684 ns 22.993 ns]
hash/metrohash/str_256      time:   [40.409 ns 40.987 ns 41.588 ns]
hash/metrohash/str_1024     time:   [83.820 ns 84.435 ns 85.412 ns]
hash/metrohash/str_4096     time:   [244.20 ns 244.74 ns 245.35 ns]
hash/metrohash/u64          time:   [5.8145 ns 5.8449 ns 5.8747 ns]
hash/metrohash/object       time:   [54.658 ns 54.812 ns 54.969 ns]

hash/seahash/str_2          time:   [14.032 ns 14.202 ns 14.391 ns]
hash/seahash/str_8          time:   [14.232 ns 14.430 ns 14.680 ns]
hash/seahash/str_16         time:   [14.614 ns 14.889 ns 15.233 ns]
hash/seahash/str_64         time:   [22.120 ns 22.508 ns 23.026 ns]
hash/seahash/str_256        time:   [50.540 ns 51.498 ns 52.459 ns]
hash/seahash/str_1024       time:   [141.30 ns 141.91 ns 142.74 ns]
hash/seahash/str_4096       time:   [498.73 ns 499.36 ns 500.06 ns]
hash/seahash/u64            time:   [6.1498 ns 6.1878 ns 6.2272 ns]
hash/seahash/object         time:   [69.752 ns 69.935 ns 70.117 ns]

hash/ahash/str_2            time:   [11.975 ns 12.208 ns 12.528 ns]
hash/ahash/str_8            time:   [11.764 ns 12.017 ns 12.310 ns]
hash/ahash/str_16           time:   [11.611 ns 11.847 ns 12.165 ns]
hash/ahash/str_64           time:   [20.693 ns 21.243 ns 21.948 ns]
hash/ahash/str_256          time:   [38.064 ns 38.856 ns 39.673 ns]
hash/ahash/str_1024         time:   [82.701 ns 82.959 ns 83.318 ns]
hash/ahash/str_4096         time:   [294.13 ns 295.45 ns 297.15 ns]
hash/ahash/u64              time:   [2.1307 ns 2.1529 ns 2.1725 ns]
hash/ahash/object           time:   [25.243 ns 25.384 ns 25.521 ns]
```

## License
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [BSD 2-Clause license](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).
