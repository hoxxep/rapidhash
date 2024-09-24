use criterion::{Bencher, Criterion};

use crate::int;
use crate::string;
use crate::object;

/// Benchmark each hashing algorithm with various input sizes.
pub fn bench(c: &mut Criterion) {
    let groups: &[(
        &str,
        Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn() -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn() -> Box<dyn FnMut(&mut Bencher)>>,
    )] = &[
        ("hash/rapidhash", Box::new(string::bench_rapidhash), Box::new(int::bench_rapidhash), Box::new(object::bench_rapidhash)),
        ("hash/rapidhash_raw", Box::new(string::bench_rapidhash_raw), Box::new(int::bench_rapidhash_raw), Box::new(object::bench_rapidhash)),
        ("hash/default", Box::new(string::bench_default), Box::new(int::bench_default), Box::new(object::bench_default)),
        ("hash/fxhash", Box::new(string::bench_fxhash), Box::new(int::bench_fxhash), Box::new(object::bench_fxhash)),
        ("hash/t1ha", Box::new(string::bench_t1ha), Box::new(int::bench_t1ha), Box::new(object::bench_t1ha)),
        ("hash/wyhash", Box::new(string::bench_wyhash), Box::new(int::bench_wyhash), Box::new(object::bench_wyhash)),
        ("hash/wyhash_raw", Box::new(string::bench_wyhash_raw), Box::new(int::bench_wyhash_raw), Box::new(object::bench_wyhash)),
        ("hash/xxhash", Box::new(string::bench_xxhash), Box::new(int::bench_xxhash), Box::new(object::bench_xxhash)),
        ("hash/metrohash", Box::new(string::bench_metrohash), Box::new(int::bench_metrohash), Box::new(object::bench_metrohash)),
        ("hash/seahash", Box::new(string::bench_seahash), Box::new(int::bench_seahash), Box::new(object::bench_seahash)),
    ];

    let sizes = [2usize, 8, 16, 64, 256, 1024, 4096];

    for (name, string_fn, int_fn, object_fn) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for size in sizes {
            let name = "str_".to_string() + &size.to_string();
            group.bench_function(name, string_fn(size));
        }
        group.bench_function("u64", int_fn());

        if name.ends_with("_raw") {
            continue;  // cannot hash objects with raw impls
        }
        group.bench_function("object", object_fn());
    }
}
