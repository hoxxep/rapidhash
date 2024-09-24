# rapidhash - native rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, which itself is the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

Memory safe, dependency free, no-std, non-cryptographic hash function. Not suitable where hashing DOS-protection is required.

From the C++ implementation:
> Passes all tests in both SMHasher and SMHasher3, collision-based study showed a collision probability lower than wyhash and close to ideal.

## Usage

```rust
use std::hash::Hasher;
use rapidhash::{rapidhash, RapidHasher};

// a std::hash::Hasher compatible hasher
let mut hasher = RapidHasher::default();
hasher.write(b"hello world");
assert_eq!(hasher.finish(), 17498481775468162579);

// raw usage
assert_eq!(rapidhash(b"hello world"), 17498481775468162579);
```

## TODO
This repo is an active work in progress.

- [x] Implement the basic `rapidhash` function.
- [ ] Add rapidhash protected variant.
- [x] Evaluate building a `RapidHasher` for the `std::hash::Hasher` trait.
- [x] Add more tests, benchmark comparisons, and further docs.
- [ ] Benchmark against the C++ implementation and confirm outputs match exactly.
- [ ] Benchmark graphs, and benchmark on x86_64 server platforms.
- [ ] License the code under a permissive license. Need to review whether this repo can be more permissive than the BSD 2-Clause the C++ crate is under.
- [ ] Publish to crates.io. (Currently in the process of requesting the rapidhash crate name, which currently is not this hash function.)

## Benchmarks
Initial benchmarks on M1 Max (aarch64) for various input sizes.

```text
crate/input_size        time:   [min       median    max      ]

rapidhash/8             time:   [11.680 ns 11.952 ns 12.266 ns]
rapidhash/16            time:   [11.572 ns 11.723 ns 11.909 ns]
rapidhash/64            time:   [18.102 ns 18.288 ns 18.493 ns]
rapidhash/256           time:   [37.611 ns 38.702 ns 40.131 ns]
rapidhash/1024          time:   [56.773 ns 57.191 ns 57.885 ns]
rapidhash/4096          time:   [168.96 ns 169.35 ns 169.77 ns]

# rapidhash::rapidhash, without the std::hash::Hasher trait overhead
rapidhash_raw/8         time:   [11.540 ns 11.822 ns 12.195 ns]
rapidhash_raw/16        time:   [11.712 ns 11.933 ns 12.197 ns]
rapidhash_raw/64        time:   [17.763 ns 18.251 ns 18.879 ns]
rapidhash_raw/256       time:   [37.636 ns 38.399 ns 39.164 ns]
rapidhash_raw/1024      time:   [56.629 ns 57.154 ns 58.157 ns]
rapidhash_raw/4096      time:   [168.79 ns 169.26 ns 169.76 ns]

default/8               time:   [15.416 ns 15.696 ns 16.013 ns]
default/16              time:   [16.633 ns 16.874 ns 17.168 ns]
default/64              time:   [29.614 ns 30.128 ns 30.704 ns]
default/256             time:   [94.702 ns 95.833 ns 96.970 ns]
default/1024            time:   [312.52 ns 319.25 ns 333.32 ns]
default/4096            time:   [1.2099 µs 1.2122 µs 1.2147 µs]

fxhash/8                time:   [10.136 ns 10.383 ns 10.670 ns]
fxhash/16               time:   [10.355 ns 10.571 ns 10.821 ns]
fxhash/64               time:   [19.295 ns 19.621 ns 19.991 ns]
fxhash/256              time:   [47.209 ns 48.230 ns 49.219 ns]
fxhash/1024             time:   [198.84 ns 199.50 ns 200.30 ns]
fxhash/4096             time:   [801.10 ns 802.85 ns 804.72 ns]

t1ha/8                  time:   [12.039 ns 12.319 ns 12.638 ns]
t1ha/16                 time:   [11.819 ns 12.099 ns 12.446 ns]
t1ha/64                 time:   [20.800 ns 21.106 ns 21.456 ns]
t1ha/256                time:   [38.075 ns 39.219 ns 40.378 ns]
t1ha/1024               time:   [90.607 ns 91.188 ns 91.973 ns]
t1ha/4096               time:   [309.10 ns 311.11 ns 314.08 ns]

wyhash/8                time:   [12.134 ns 12.394 ns 12.684 ns]
wyhash/16               time:   [12.408 ns 12.579 ns 12.789 ns]
wyhash/64               time:   [17.961 ns 18.125 ns 18.316 ns]
wyhash/256              time:   [36.161 ns 36.916 ns 37.688 ns]
wyhash/1024             time:   [68.004 ns 68.844 ns 70.326 ns]
wyhash/4096             time:   [224.97 ns 225.42 ns 225.97 ns]

# wyhash::wyhash, without the std::hash::Hasher trait overhead
wyhash_raw/8            time:   [11.274 ns 11.507 ns 11.777 ns]
wyhash_raw/16           time:   [11.677 ns 11.889 ns 12.149 ns]
wyhash_raw/64           time:   [17.401 ns 17.570 ns 17.764 ns]
wyhash_raw/256          time:   [35.592 ns 36.582 ns 37.599 ns]
wyhash_raw/1024         time:   [68.634 ns 69.488 ns 70.728 ns]
wyhash_raw/4096         time:   [221.29 ns 222.24 ns 223.66 ns]

xxhash/8                time:   [17.829 ns 18.094 ns 18.398 ns]
xxhash/16               time:   [20.159 ns 20.462 ns 20.873 ns]
xxhash/64               time:   [24.943 ns 25.171 ns 25.436 ns]
xxhash/256              time:   [44.993 ns 45.597 ns 46.194 ns]
xxhash/1024             time:   [67.260 ns 68.012 ns 69.004 ns]
xxhash/4096             time:   [182.22 ns 182.86 ns 183.50 ns]

metrohash/8             time:   [14.429 ns 14.669 ns 14.953 ns]
metrohash/16            time:   [14.746 ns 14.956 ns 15.231 ns]
metrohash/64            time:   [22.262 ns 22.546 ns 22.876 ns]
metrohash/256           time:   [41.037 ns 42.317 ns 43.908 ns]
metrohash/1024          time:   [84.311 ns 84.837 ns 85.486 ns]
metrohash/4096          time:   [244.28 ns 244.83 ns 245.47 ns]

seahash/8               time:   [14.247 ns 14.522 ns 14.856 ns]
seahash/16              time:   [14.452 ns 14.641 ns 14.876 ns]
seahash/64              time:   [22.007 ns 22.222 ns 22.478 ns]
seahash/256             time:   [49.987 ns 50.748 ns 51.505 ns]
seahash/1024            time:   [141.59 ns 141.84 ns 142.15 ns]
seahash/4096            time:   [498.97 ns 499.74 ns 500.62 ns]
```
