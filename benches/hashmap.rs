use criterion::{Bencher, Criterion};
use rand::distributions::Alphanumeric;
use rand::Rng;

/// Benchmark each hashing algorithm with hashmaps.
pub fn bench(c: &mut Criterion) {
    let groups: &[(
        &str,
        Box<dyn Fn(usize, usize, usize) -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>,
    )] = &[
        ("map/rapidhash", Box::new(bench_rapidhash), Box::new(bench_rapidhash_u64)),
        ("map/default", Box::new(bench_default), Box::new(bench_default_u64)),
        ("map/fxhash", Box::new(bench_fxhash), Box::new(bench_fxhash_u64)),
    ];

    let string_sizes = [
        (1000, 4, 4),
        (1000, 4, 64),
    ];

    let int_sizes = [
        100000,
    ];

    for (name, strings, ints) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for (size, min, max) in string_sizes {
            let name = format!("{}_{}_{}", size, min, max);
            group.bench_function(name, strings(size, min, max));
        }
        for size in int_sizes {
            let name = format!("{}_u64", size);
            group.bench_function(name, ints(size));
        }
    }
}

fn sample(count: usize, min: usize, max: usize) -> Vec<String> {
    (0..count)
        .map(|_| {
            let len = rand::thread_rng().gen_range(min..=max);
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(len)
                .map(char::from)
                .collect();
            s
        })
        .collect()
}

fn sample_u64(count: usize) -> Vec<u64> {
    (0..count)
        .map(|_| rand::random())
        .collect()
}

fn bench_rapidhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            sample(count, min, max)
        }, |strings: Vec<String>| {
            let mut map = rapidhash::RapidHashMap::default();
            for string in strings {
                let len = string.len();
                map.insert(string, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_rapidhash_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            sample_u64(count)
        }, |ints: Vec<u64>| {
            let mut map = rapidhash::RapidHashMap::default();
            for int in ints {
                let len = int >> 3;
                map.insert(int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_default(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            sample(count, min, max)
        }, |strings: Vec<String>| {
            let mut map = std::collections::HashMap::new();
            for string in strings {
                let len = string.len();
                map.insert(string, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_default_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            sample_u64(count)
        }, |ints: Vec<u64>| {
            let mut map = std::collections::HashMap::new();
            for int in ints {
                let len = int >> 3;
                map.insert(int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_fxhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            sample(count, min, max)
        }, |strings: Vec<String>| {
            let mut map = fxhash::FxHashMap::default();
            for string in strings {
                let len = string.len();
                map.insert(string, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_fxhash_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched(|| {
            sample_u64(count)
        }, |ints: Vec<u64>| {
            let mut map = fxhash::FxHashMap::default();
            for int in ints {
                let len = int >> 3;
                map.insert(int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}
