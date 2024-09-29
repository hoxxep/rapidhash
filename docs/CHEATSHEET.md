# Repository Command Cheatsheet

## Development
```shell
# Run tests
cargo test --all-features

# Run tests, for no_std with std = off and unsafe = off
cargo test --no-default-features --lib

# Check MSRV
cargo +1.77.0 test --all-features

# Run all benchmarks (assumes cargo-criterion is installed)
cargo criterion --bench bench --all-features

# Run all benchmarks, but unsafe=disabled
cargo criterion --bench bench --features rng

# Run quality tests across various hash functions
cargo bench --bench quality --all-features
```

## Fuzzing
```shell
# fuzz the raw rapidhash method. (assumes cargo-fuzz is installed)
cargo +nightly fuzz run --features unsafe rapidhash

# fuzz the RapidHasher struct with std::hash::Hasher write and finish calls.
cargo +nightly fuzz run --features unsafe rapidhasher

# use AFL fuzzing. (assumes cargo-afl is installed)
cargo afl fuzz -i in -o out target/debug/afl_rapidhash
```

## Documentation
```shell
# Install cargo-docs
cargo install cargo-docs

# Preview the documentation
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly docs -- --all-features
```

## CLI
```shell
# From stdin
echo "example" | cargo run --example cli

# From file
cargo run --example cli -- example.txt
```
