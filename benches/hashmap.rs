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
        ("map/rapidhash_inline", Box::new(bench_rapidhash_inline), Box::new(bench_rapidhash_inline_u64)),
        ("map/default", Box::new(bench_default), Box::new(bench_default_u64)),
        ("map/fxhash", Box::new(bench_fxhash), Box::new(bench_fxhash_u64)),
        ("map/gxhash", Box::new(bench_gxhash), Box::new(bench_gxhash_u64)),
    ];

    let string_sizes = [
        (1000, 4, 4),
        (1000, 4, 64),
        (0, 0, 0),
    ];

    let int_sizes = [
        100000,
    ];

    for (name, strings, ints) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for (size, min, max) in string_sizes {
            let name = if size != 0 {
                format!("{}_{}_{}", size, min, max)
            } else {
                "450000_words".to_string()
            };
            group.bench_function(name, strings(size, min, max));
        }
        for size in int_sizes {
            let name = format!("{}_u64", size);
            group.bench_function(name, ints(size));
        }
    }
}

lazy_static::lazy_static! {
    static ref WORDS: Vec<String> = {
        const WORDS_FILE: &str = "target/words.txt";
        let text: String = if std::fs::exists(WORDS_FILE).unwrap_or(false) {
            println!("Reading dictionary words from {WORDS_FILE}");
            std::fs::read_to_string(WORDS_FILE).expect("Failed to read words from text file.")
        } else {
            println!("Downloading ~1.5MB of dictionary words from github...");
            let text = reqwest::blocking::get("https://raw.githubusercontent.com/dwyl/english-words/refs/heads/master/words.txt")
                .expect("Could not fetch dictionary words from github")
                .text().expect("Could not read downloaded dictionary words");
            println!("Caching dictionary words to {WORDS_FILE}");
            std::fs::write(WORDS_FILE, &text).expect("Could not write dictionary words to text file.");
            text
        };

        let words: Vec<_> = text.lines().map(str::to_string).collect();
        assert!(words.len() > 450_000 && words.len() < 480_000, "Unexpected number of dictionary words");
        words
    };
}

fn sample(count: usize, min: usize, max: usize) -> Vec<String> {
    if count == 0 {
        return WORDS.clone();
    }

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
        .map(|_| rand::thread_rng().gen_range(0..500000))
        .collect()
}

/// Use .iter_batched_ref to avoid paying the HashMap destruction cost.
fn bench_rapidhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidHashMap::default(), sample(count, min, max))
        }, |(map, strings)| {
            for string in strings {
                let len = string.len();
                map.insert(string.clone(), len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_rapidhash_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidHashMap::default(), sample_u64(count))
        }, |(map, ints)| {
            for int in ints {
                let len = *int >> 3;
                map.insert(*int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_rapidhash_inline(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidInlineHashMap::default(), sample(count, min, max))
        }, |(map, strings)| {
            for string in strings {
                let len = string.len();
                map.insert(string.clone(), len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_rapidhash_inline_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidInlineHashMap::default(), sample_u64(count))
        }, |(map, ints)| {
            for int in ints {
                let len = *int >> 3;
                map.insert(*int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_default(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashMap::new(), sample(count, min, max))
        }, |(map, strings)| {
            for string in strings {
                let len = string.len();
                map.insert(string.clone(), len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_default_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashMap::new(), sample_u64(count))
        }, |(map, ints)| {
            for int in ints {
                let len = *int >> 3;
                map.insert(*int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_fxhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (fxhash::FxHashMap::default(), sample(count, min, max))
        }, |(map, strings)| {
            for string in strings {
                let len = string.len();
                map.insert(string.clone(), len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_fxhash_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (fxhash::FxHashMap::default(), sample_u64(count))
        }, |(map, ints)| {
            for int in ints {
                let len = *int >> 3;
                map.insert(*int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_gxhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (gxhash::HashMap::default(), sample(count, min, max))
        }, |(map, strings)| {
            for string in strings {
                let len = string.len();
                map.insert(string.clone(), len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_gxhash_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (gxhash::HashMap::default(), sample_u64(count))
        }, |(map, ints)| {
            for int in ints {
                let len = *int >> 3;
                map.insert(*int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}
