# Fuzzing

Read the [cargo fuzzing guide](https://rust-fuzz.github.io/book/introduction.html) to get started.

## Example commands

```shell
# fuzz the raw rapidhash method.
cargo +nightly fuzz run --features unsafe rapidhash

# fuzz the RapidHasher struct with std::hash::Hasher write and finish calls.
cargo +nightly fuzz run --features unsafe rapidhasher

# use AFL fuzzing.
cargo afl fuzz -i in -o out target/debug/afl_rapidhash
```
