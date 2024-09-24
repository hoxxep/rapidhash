use criterion::{Bencher, Criterion};
use rand::distributions::Alphanumeric;
use rand::Rng;

/// Benchmark each hashing algorithm with hashmaps.
pub fn bench(c: &mut Criterion) {
    let groups: &[(&str, Box<dyn Fn(usize, usize, usize) -> Box<dyn FnMut(&mut Bencher)>>)] = &[
        ("map/rapidhash", Box::new(bench_rapidhash)),
        ("map/default", Box::new(bench_default)),
        ("map/fxhash", Box::new(bench_fxhash)),
    ];

    let sizes = [
        (1000, 4, 4),
        (1000, 4, 64),
    ];

    for (name, function) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for (size, min, max) in sizes {
            let name = format!("{}_{}_{}", size, min, max);
            group.bench_function(name, function(size, min, max));
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
