# rapidhash - rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

- **High quality**, the fastest hash passing all tests in the SMHasher and SMHasher3 benchmark. Collision-based study showed a collision probability lower than wyhash, foldhash, and close to ideal.
- **Very fast**, the fastest passing hash in SMHasher3. Significant peak throughput improvement over wyhash and foldhash. Fastest memory-safe hash. Fastest platform-independent hash. Fastest const hash.
- **Platform independent**, works on all platforms, no dependency on machine-specific vectorized or cryptographic hardware instructions. Optimised for both AMD64 and AArch64.
- **Memory safe**, when the `unsafe` feature is disabled (default). This implementation has also been fuzz-tested with `cargo fuzz`.
- **No dependencies and no-std compatible** when disabling default features.
- **Official successor to wyhash** with improved speed, quality, and compatibility.
- **Run-time and compile-time hashing** as the hash implementation is fully `const`.
- **Idiomatic** `std::hash::Hasher` compatible hasher for `HashMap` and `HashSet` usage.
- **Non-cryptographic** hash function that's "minimally DoS resistant" in the same manner as foldhash.

**Sponsored by [Upon](https://uponvault.com?utm_source=github&utm_campaign=rapidhash)**, inheritance vaults for your digital life. Ensure your family can access your devices, accounts, and assets when the unexpected happens.

## Usage
### Portable Hashing
Full compatibility with C++ rapidhash algorithms, methods are provided for all rapidhash V1, V2, and V3 (with micro/nano) variants. These are stable functions whose output will not change between crate versions.

```rust
use std::hash::{BuildHasher, Hasher};
use rapidhash::v3::{rapidhash_v3_seeded, RapidSecrets};

/// Set your global hashing secrets.
///
/// - For HashDoS resistance, choose a randomised secret.
/// - For C++ compatibility, use the `seed_cpp` method or default secrets.
const RAPID_SECRETS: RapidSecrets = RapidSecrets::seed(0x123456);

/// Make a helper function that sets your rapidhash version and secrets.
#[inline]
pub fn rapidhash(data: &[u8]) -> u64 {
    rapidhash_v3_seeded(data, &RAPID_SECRETS)
}

assert_eq!(rapidhash(b"hello world"), 11653223729569656151);
```

Please see the [`portable-hash` crate](https://github.com/hoxxep/portable-hash) using the standard library hashing traits is not recommended for portable hashing. Rapidhash is planning to implement the `PortableHash` and `PortableHasher` traits in a future release.

### In-Memory Hashing
Following rust's `std::hash` traits, the underlying hash function may change between minor versions, and is only suitable for in-memory hashing. These types are optimised for speed and minimal DoS resistance, available in the `rapidhash::fast` and `rapidhash::quality` flavours.

- `RapidHasher`: A `std::hash::Hasher` compatible hasher that uses the rapidhash algorithm.
- `RapidHashBuilder`: A `std::hash::BuildHasher` for initialising the hasher with the default seed and secrets.
- `RandomState`: A `std::hash::BuildHasher` for initialising the hasher with a random seed and secrets.
- `RapidHashMap` and `RapidHashSet`: Helper types for using `RapidHasher` with `HashMap` and `HashSet`.

```rust
use rapidhash::fast::RapidHashMap;

// A HashMap using RapidHasher for fast in-memory hashing.
let mut map = RapidHashMap::default();
map.insert("key", "value");
```

```rust
use std::hash::BuildHasher;
use rapidhash::quality::RapidBuildHasher;

// Using the RapidHasher directly for in-memory hashing.
let hasher = RapidBuildHasher::default();
assert_eq!(hasher.hash_one(b"hello world"), 1790036888308448300);
```

### CLI
Rapidhash can also be installed as a CLI tool to hash files or stdin. This is not a cryptographic hash, but should be much faster than cryptographic hashes. This is fully compatible with the C++ rapidhash V1, V2, and V3 algorithms.

Output is the decimal string of the `u64` hash value.

```shell
# install
cargo install rapidhash

# hash a file (output: 8543579700415218186)
rapidhash --v3 example.txt

# hash stdin (output: 8543579700415218186)
echo "example" | rapidhash --v3
```

## Features

- `default`: `std`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables using the `rand` library to more securely initialise `RandomState`. Includes the `rand` crate dependency.
- `rng`: Enables `RapidRng`, a fast, non-cryptographic PRNG based on rapidrng. Includes the `rand_core` crate dependency.
- `unsafe`: Uses unsafe pointer arithmetic to skip some unnecessary bounds checks for a small 3-4% performance improvement.

## Benchmarks

Initial benchmarks on M1 Max (aarch64) for various input sizes.

### Hashing Benchmarks

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash.svg)

<details>
<summary><strong>Comparison to foldhash</strong></summary>

- Rapidhash is generally faster with string and byte inputs.
- Foldhash is generally faster with integer tuples by using a 128bit buffer for integer inputs.

```txt
             ┌────────────────┬─────────────┬────────────┐
             │         metric ┆ rapidhash-f ┆ foldhash-f │
             │            --- ┆         --- ┆        --- │
             │            str ┆         f64 ┆        f64 │
             ╞════════════════╪═════════════╪════════════╡
             │       avg_rank ┆        1.50 ┆       1.50 │
             │ geometric_mean ┆        4.72 ┆       4.83 │
             └────────────────┴─────────────┴────────────┘
┌────────────────┬────────────┬─────────────┬────────────┐
│          distr ┆      bench ┆ rapidhash-f ┆ foldhash-f │
│            --- ┆        --- ┆         --- ┆        --- │
│            str ┆        str ┆         f64 ┆        f64 │
╞════════════════╪════════════╪═════════════╪════════════╡
│            u32 ┆   hashonly ┆        0.56 ┆       0.62 │
│            u32 ┆ lookupmiss ┆        1.40 ┆       1.51 │
│            u32 ┆  lookuphit ┆        1.76 ┆       1.84 │
│            u32 ┆   setbuild ┆        3.86 ┆       4.07 │
│        u32pair ┆   hashonly ┆        0.85 ┆       0.62 │
│        u32pair ┆ lookupmiss ┆        1.88 ┆       1.59 │
│        u32pair ┆  lookuphit ┆        2.25 ┆       1.88 │
│        u32pair ┆   setbuild ┆        4.52 ┆       4.28 │
│            u64 ┆   hashonly ┆        0.81 ┆       0.62 │
│            u64 ┆ lookupmiss ┆        1.77 ┆       1.46 │
│            u64 ┆  lookuphit ┆        2.08 ┆       1.83 │
│            u64 ┆   setbuild ┆        4.32 ┆       4.10 │
│      u64lobits ┆   hashonly ┆        0.81 ┆       0.62 │
│      u64lobits ┆ lookupmiss ┆        1.73 ┆       1.46 │
│      u64lobits ┆  lookuphit ┆        2.00 ┆       1.81 │
│      u64lobits ┆   setbuild ┆        4.18 ┆       4.02 │
│      u64hibits ┆   hashonly ┆        0.81 ┆       0.62 │
│      u64hibits ┆ lookupmiss ┆        1.71 ┆       1.46 │
│      u64hibits ┆  lookuphit ┆        2.12 ┆       1.80 │
│      u64hibits ┆   setbuild ┆        4.04 ┆       4.05 │
│        u64pair ┆   hashonly ┆        1.31 ┆       0.78 │
│        u64pair ┆ lookupmiss ┆        2.52 ┆       1.84 │
│        u64pair ┆  lookuphit ┆        2.91 ┆       2.14 │
│        u64pair ┆   setbuild ┆        5.18 ┆       4.33 │
│           ipv4 ┆   hashonly ┆        0.55 ┆       0.62 │
│           ipv4 ┆ lookupmiss ┆        1.45 ┆       1.52 │
│           ipv4 ┆  lookuphit ┆        1.77 ┆       1.83 │
│           ipv4 ┆   setbuild ┆        4.02 ┆       4.05 │
│           ipv6 ┆   hashonly ┆        0.83 ┆       0.78 │
│           ipv6 ┆ lookupmiss ┆        1.81 ┆       1.74 │
│           ipv6 ┆  lookuphit ┆        2.55 ┆       2.39 │
│           ipv6 ┆   setbuild ┆        4.44 ┆       4.32 │
│           rgba ┆   hashonly ┆        1.25 ┆       0.63 │
│           rgba ┆ lookupmiss ┆        2.52 ┆       1.71 │
│           rgba ┆  lookuphit ┆        3.28 ┆       2.51 │
│           rgba ┆   setbuild ┆        5.90 ┆       4.72 │
│ strenglishword ┆   hashonly ┆        1.38 ┆       6.29 │
│ strenglishword ┆ lookupmiss ┆        4.26 ┆       6.65 │
│ strenglishword ┆  lookuphit ┆        8.73 ┆      10.92 │
│ strenglishword ┆   setbuild ┆       14.78 ┆      17.22 │
│        struuid ┆   hashonly ┆        2.75 ┆       5.51 │
│        struuid ┆ lookupmiss ┆        6.42 ┆       8.01 │
│        struuid ┆  lookuphit ┆        9.95 ┆      12.05 │
│        struuid ┆   setbuild ┆       13.82 ┆      16.29 │
│         strurl ┆   hashonly ┆        5.01 ┆       7.38 │
│         strurl ┆ lookupmiss ┆        8.01 ┆       9.89 │
│         strurl ┆  lookuphit ┆       14.45 ┆      16.15 │
│         strurl ┆   setbuild ┆       21.27 ┆      22.78 │
│        strdate ┆   hashonly ┆        1.33 ┆       5.47 │
│        strdate ┆ lookupmiss ┆        4.40 ┆       6.29 │
│        strdate ┆  lookuphit ┆        6.42 ┆       8.01 │
│        strdate ┆   setbuild ┆        9.76 ┆      12.54 │
│      accesslog ┆   hashonly ┆        1.55 ┆       1.16 │
│      accesslog ┆ lookupmiss ┆        2.93 ┆       2.26 │
│      accesslog ┆  lookuphit ┆        4.06 ┆       3.21 │
│      accesslog ┆   setbuild ┆        6.46 ┆       5.48 │
│       kilobyte ┆   hashonly ┆       31.78 ┆      31.93 │
│       kilobyte ┆ lookupmiss ┆       35.09 ┆      33.73 │
│       kilobyte ┆  lookuphit ┆       72.32 ┆      76.98 │
│       kilobyte ┆   setbuild ┆      101.35 ┆     114.84 │
│    tenkilobyte ┆   hashonly ┆      235.44 ┆     314.27 │
│    tenkilobyte ┆ lookupmiss ┆      243.00 ┆     317.11 │
│    tenkilobyte ┆  lookuphit ┆      608.65 ┆     683.77 │
│    tenkilobyte ┆   setbuild ┆     1034.19 ┆    1079.70 │
└────────────────┴────────────┴─────────────┴────────────┘
```

</details>

<details>
<summary><strong>Benchmark notes</strong></summary>

- Hash throughput/latency does not measure hash "quality", and many of the benchmarked functions fail SMHasher3 quality tests. Hash quality affects hashmap performance, as well as algorithms that benefit from high quality hash functions such as HyperLogLog and MinHash.
- Most hash functions will be affected heavily by whether the compiler has inlined them. Rapidhash tries very hard to always be inlined by the compiler, but the larger a program or benchmark gets, the less likely it is to be inlined due to Rust's `BuildHasher::hash_one` method not being `#[inline(always)]`.
- `gxhash` has high throughput by using AES instructions. It's a great hash function, but is not a portable hash function (often requires `target-cpu=native` to compile), uses unsafe code, and is not minimally DoS resistant.
- Benchmark your own use case, with your real world dataset! We suggest experimenting with rapidhash, foldhash, and gxhash to see what works for you. Different inputs will benefit from different hash functions.

</details>

## Rapidhash Versions

### Portable Hashing
Fixed versioning with C++ compatibility is presented in `rapidhash::v1`, `rapidhash::v2`, and `rapidhash::v3` modules.

Rapidhash V3 is the recommended, fastest, and most recent version of the hash. Others are provided for backwards compatibility.

### In-Memory Hashing
Rust hasing traits (`RapidHasher`, `RapidBuildHasher`, etc.) are implemented in `rapidhash::fast`, `rapidhash::quality`, and `rapidhash::inner` modules. These are not guaranteed to give a consistent hash output between platforms, compiler versions, or crate versions as the rust `Hasher` trait [is not suitable](https://github.com/hoxxep/portable-hash/?tab=readme-ov-file#whats-wrong-with-the-stdhash-traits) for portable hashing.

- Use `rapidhash::fast` for optimal hashing speed with a slightly lower hash quality. Best for most datastructures such as HashMap and HashSet usage.
- Use `rapidhash::quality` where statistical hash quality is the priority, such as HyperLogLog or MinHash algorithms.
- Use `rapidhash::inner` to configure advanced parameters to configure the hash function specifically to your use case. This allows tweaking the following compile time parameters, which all change the hash output:
    - `AVALANCHE`: Enables the final avalanche mixing step to improve hash quality. Enabled on quality, disabled on fast.
    - `SPONGE`: Hash integer types by collecting them into a 128-bit buffer and mixing them together, rather than hashing each integer individually. If this is disabled, it will perform a folded multiply for each integer. Enabled by default.
    - `COMPACT`: Generates fewer instructions at compile time by reducing the manual loop unrolling. This might improve the probability of rapidhash being inlined, but may be slower on some platforms. Disabled by default.
    - `PROTECTED`: Slightly stronger hash quality and DoS resistance by performing two extra XOR instructions on every mix step. Disabled by default.

## Versioning
The minimum supported Rust version (MSRV) is 1.77.0.

The rapidhash crate follows the following versioning scheme:
- Major for breaking API changes and MSRV version bumps or any changes to `rapidhash_v*` method output.
- Minor for significant API additions/deprecations or any changes to `RapidHasher` output.
- Patch for bug fixes and performance improvements.

Portable hash outputs (eg. `rapidhash_v3`) are guaranteed to be stable. In-memory hash outputs (eg. `RapidHasher`) may change between minor versions to allow us to freely improve performance.

## License and Acknowledgements
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to [Nicolas De Carli](https://github.com/Nicoshev) for the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [MIT License](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).

With thanks to [Justin Bradford](https://github.com/jabr) for letting us use the rapidhash crate name 🍻
