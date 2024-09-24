# rapidhash - native rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, which itself is the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

Memory safe, dependency free, and no-std compatible.

## Usage

```rust
use rapidhash::rapidhash;

assert_eq!(rapidhash(b"hello world"), 17498481775468162579);
```

## TODO
This repo is an active work in progress.

- [x] Implement the basic `rapidhash` function.
- [ ] Add rapidhash protected variant.
- [ ] Evaluate building a `RapidHasher` for the `std::hash::Hasher` trait.
- [ ] Benchmark against the C++ implementation and confirm outputs match exactly.
- [ ] Add more tests, benchmark comparisons, and further docs.
- [ ] License the code under a permissive license. Need to review whether this repo can be more permissive than the BSD 2-Clause the C++ crate is under.
- [ ] Publish to crates.io. (Confusingly, there is a rapidhash crate that is not this hash function.)

## Benchmarks
Initial benchmarks on M1 Max (aarch64) for various input sizes.

```text
crate/input_size        time:   [min       median    max      ]

# currently without the std::hash::Hasher trait overhead
rapidhash/8             time:   [12.070 ns 12.412 ns 12.805 ns]
rapidhash/16            time:   [11.916 ns 12.173 ns 12.482 ns]
rapidhash/64            time:   [18.683 ns 18.946 ns 19.239 ns]
rapidhash/256           time:   [37.639 ns 38.398 ns 39.139 ns]
rapidhash/1024          time:   [57.908 ns 58.396 ns 59.024 ns]
rapidhash/4096          time:   [172.06 ns 173.17 ns 174.48 ns]

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

# wyhash without the std::hash::Hasher trait overhead
wyhash_raw/8            time:   [11.274 ns 11.507 ns 11.777 ns]
wyhash_raw/16           time:   [11.677 ns 11.889 ns 12.149 ns]
wyhash_raw/64           time:   [17.401 ns 17.570 ns 17.764 ns]
wyhash_raw/256          time:   [35.592 ns 36.582 ns 37.599 ns]
wyhash_raw/1024         time:   [68.634 ns 69.488 ns 70.728 ns]
wyhash_raw/4096         time:   [221.29 ns 222.24 ns 223.66 ns]
```
