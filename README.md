# rapidhash - native rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, which itself is the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

## Usage

```rust
use rapidhash::rapidhash;

assert_eq!(rapidhash(b"hello world"), 17498481775468162579);
```

## TODO
This repo is an active work in progress.

- [x] Implement the basic `rapidhash` function.
- [ ] Evaluate building a `RapidHasher` for the `std::hash::Hasher` trait.
- [ ] Benchmark against the C++ implementation and confirm outputs match exactly.
- [ ] Add more tests, benchmark comparisons, and further docs.
- [ ] License the code under a permissive license. Need to review whether this repo can be more permissive than the BSD 2-Clause the C++ crate is under.
- [ ] Publish to crates.io. (Confusingly, there is a rapidhash crate that is not this hash function.)
