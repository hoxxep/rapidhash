use criterion::{Bencher, Criterion, Throughput};

use crate::int;
use crate::vector;
use crate::object;

/// Benchmark each hashing algorithm with various input sizes.
///
/// TODO: small and large object benchmarks.
///     examples: hashing a key for HashMap vs. hashing a large value for HashSet
pub fn bench(c: &mut Criterion) {
    let groups: &[(
        &str,
        Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn() -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn() -> Box<dyn FnMut(&mut Bencher)>>,
    )] = &[
        ("hash/rapidhash", Box::new(vector::bench_rapidhash), Box::new(int::bench_rapidhash), Box::new(object::bench_rapidhash)),
        ("hash/rapidhash_raw", Box::new(vector::bench_rapidhash_raw), Box::new(int::bench_rapidhash_raw), Box::new(object::bench_rapidhash)),
        ("hash/default", Box::new(vector::bench_default), Box::new(int::bench_default), Box::new(object::bench_default)),
        ("hash/fxhash", Box::new(vector::bench_fxhash), Box::new(int::bench_fxhash), Box::new(object::bench_fxhash)),
        ("hash/gxhash", Box::new(vector::bench_gxhash), Box::new(int::bench_gxhash), Box::new(object::bench_gxhash)),
        ("hash/ahash", Box::new(vector::bench_ahash), Box::new(int::bench_ahash), Box::new(object::bench_ahash)),
        ("hash/t1ha", Box::new(vector::bench_t1ha), Box::new(int::bench_t1ha), Box::new(object::bench_t1ha)),
        ("hash/wyhash", Box::new(vector::bench_wyhash), Box::new(int::bench_wyhash), Box::new(object::bench_wyhash)),
        ("hash/wyhash_raw", Box::new(vector::bench_wyhash_raw), Box::new(int::bench_wyhash_raw), Box::new(object::bench_wyhash)),
        ("hash/xxhash", Box::new(vector::bench_xxhash), Box::new(int::bench_xxhash), Box::new(object::bench_xxhash)),
        ("hash/metrohash", Box::new(vector::bench_metrohash), Box::new(int::bench_metrohash), Box::new(object::bench_metrohash)),
        ("hash/seahash", Box::new(vector::bench_seahash), Box::new(int::bench_seahash), Box::new(object::bench_seahash)),
        ("hash/farmhash", Box::new(vector::bench_farmhash), Box::new(int::bench_farmhash), Box::new(object::bench_farmhash)),
        ("hash/highwayhash", Box::new(vector::bench_highwayhash), Box::new(int::bench_highwayhash), Box::new(object::bench_highwayhash)),
        ("hash/rustc-hash", Box::new(vector::bench_rustchash), Box::new(int::bench_rustchash), Box::new(object::bench_rustchash)),
    ];

    let sizes = [2usize, 8, 16, 64, 100, 177, 256, 1024, 4096];

    for (name, string_fn, int_fn, object_fn) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for size in sizes {
            let name = "str_".to_string() + &size.to_string();
            group.throughput(Throughput::Bytes(size as u64));
            group.bench_function(name, string_fn(size));
        }

        group.throughput(Throughput::Elements(1));
        if name == &"hash/rapidhash" {
            group.bench_function("u8", int::bench_rapidhash_u8());
            group.bench_function("u16", int::bench_rapidhash_u16());
            group.bench_function("u32", int::bench_rapidhash_u32());
            group.bench_function("u64", int_fn());
            group.bench_function("u128", int::bench_rapidhash_u128());
        } else {
            group.bench_function("u64", int_fn());
        }

        if name.ends_with("_raw") {
            continue;  // cannot hash objects with raw impls
        }
        group.bench_function("object", object_fn());
        if name == &"hash/rapidhash" {
            group.bench_function("object_inline", object::bench_rapidhash_inline());
        }

    }
}
