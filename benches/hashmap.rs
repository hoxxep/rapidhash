use std::hash::BuildHasherDefault;
use criterion::{Bencher, Criterion, Throughput};
use rand::distributions::{Alphanumeric, DistString, Distribution, WeightedIndex};
use rand::Rng;
use wyhash::WyHash;

/// Benchmark each hashing algorithm with hashmaps.
pub fn bench(c: &mut Criterion) {
    let groups: &[(
        &str,
        Box<dyn Fn(usize, usize, usize) -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>,
        Box<dyn Fn(usize) -> Box<dyn FnMut(&mut Bencher)>>,
    )] = &[
        ("map/rapidhash", Box::new(bench_rapidhash), Box::new(bench_rapidhash_u64), Box::new(bench_rapidhash_object)),
        ("map/rapidhash_inline", Box::new(bench_rapidhash_inline), Box::new(bench_rapidhash_inline_u64), Box::new(bench_rapidhash_inline_object)),
        ("map/default", Box::new(bench_default), Box::new(bench_default_u64), Box::new(bench_default_object)),
        ("map/fxhash", Box::new(bench_fxhash), Box::new(bench_fxhash_u64), Box::new(bench_fxhash_object)),
        ("map/gxhash", Box::new(bench_gxhash), Box::new(bench_gxhash_u64), Box::new(bench_gxhash_object)),
        ("map/wyhash", Box::new(bench_wyhash), Box::new(bench_wyhash_u64), Box::new(bench_wyhash_object)),
    ];

    let string_sizes = [
        (1000, 4, 4, "small"),
        (10000, 10, 60, "emails"),  // estimating emails to be between 20-50 chars
        (0, 0, 0, "words"),
    ];

    let int_sizes = [
        100000,
    ];

    let obj_sizes = [
        10000,
    ];

    for (name, strings, ints, objs) in groups.into_iter() {
        let mut group = c.benchmark_group(name.to_string());
        for (size, min, max, name) in string_sizes {
            let name_size = if size == 0 { 450000 } else { size };
            let name = format!("{}_{}", name_size, name);
            group.throughput(Throughput::Elements(name_size as u64));
            group.bench_function(name, strings(size, min, max));
        }
        for size in int_sizes {
            let name = format!("{}_u64", size);
            group.throughput(Throughput::Elements(size as u64));
            group.bench_function(name, ints(size));
        }
        for size in obj_sizes {
            let name = format!("{}_struct", size);
            group.throughput(Throughput::Elements(size as u64));
            group.bench_function(name, objs(size));
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

fn sample_string(count: usize, min: usize, max: usize) -> Vec<String> {
    if count == 0 {
        return WORDS.clone();
    }

    if min == 10 && max == 60 {
        return sample_emails(count);
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

fn sample_emails(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();

    // weights roughly estimated from https://atdata.com/blog/long-email-addresses/
    let weights = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0-9
        1, 1, 2, 5, 11, 19, 36, 52, 75, 85,  // 10-19
        94, 93, 88, 77, 65, 52, 38, 27, 21, 15,  // 20-29
        11, 8, 7, 6, 5, 4, 3, 2, 2, 1,  // 30-39
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 40-49
        0, 1, 0, 0, 1, 0, 0, 0, 1, 0,  // 50-59
    ];

    let index = WeightedIndex::new(weights).unwrap();

    (0..count)
        .map(|_| {
            let length = index.sample(&mut rng);
            let address = Alphanumeric.sample_string(&mut rng, length);
            address
        })
        .collect()
}

fn sample_u64(count: usize) -> Vec<u64> {
    (0..count)
        .map(|_| rand::thread_rng().gen_range(0..500000))
        .collect()
}

/// A simple object to test with.
#[derive(Hash, PartialEq, Eq, Clone)]
struct Object {
    time_sec: u64,
    time_ns: u32,
    user_id: [u8; 16],
    url: String,
    event_source: String,
    event_data: String,
}

fn sample_object(count: usize) -> Vec<Object> {
    let mut rng = rand::thread_rng();
    let mut objects = Vec::with_capacity(count);
    for _ in 0..count {
        let url_len = rng.gen_range(30..=70);
        let event_data_len = rng.gen_range(250..=450);

        objects.push(Object {
            time_sec: rng.gen(),
            time_ns: rng.gen(),
            user_id: rng.gen(),
            url: Alphanumeric.sample_string(&mut rng, url_len),
            event_source: Alphanumeric.sample_string(&mut rng, 20),
            event_data: Alphanumeric.sample_string(&mut rng, event_data_len),
        });
    }
    objects
}

/// Use .iter_batched_ref to avoid paying the HashMap destruction cost.
fn bench_rapidhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidHashMap::default(), sample_string(count, min, max))
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

fn bench_rapidhash_object(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidHashSet::default(), sample_object(count))
        }, |(set, objs)| {
            for obj in objs {
                set.insert(obj.clone());
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_rapidhash_inline(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidInlineHashMap::default(), sample_string(count, min, max))
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

fn bench_rapidhash_inline_object(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (rapidhash::RapidInlineHashSet::default(), sample_object(count))
        }, |(set, objs)| {
            for obj in objs {
                set.insert(obj.clone());
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_default(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashMap::new(), sample_string(count, min, max))
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

fn bench_default_object(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashSet::new(), sample_object(count))
        }, |(set, objs)| {
            for obj in objs {
                set.insert(obj.clone());
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_fxhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (fxhash::FxHashMap::default(), sample_string(count, min, max))
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

fn bench_fxhash_object(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (fxhash::FxHashSet::default(), sample_object(count))
        }, |(set, objs)| {
            for obj in objs {
                set.insert(obj.clone());
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_gxhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (gxhash::HashMap::default(), sample_string(count, min, max))
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

fn bench_gxhash_object(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (gxhash::HashSet::default(), sample_object(count))
        }, |(set, objs)| {
            for obj in objs {
                set.insert(obj.clone());
            }
        }, criterion::BatchSize::LargeInput);
    })
}


fn bench_wyhash(count: usize, min: usize, max: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashMap::with_hasher(BuildHasherDefault::<WyHash>::default()), sample_string(count, min, max))
        }, |(map, strings)| {
            for string in strings {
                let len = string.len();
                map.insert(string.clone(), len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_wyhash_u64(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashMap::with_hasher(BuildHasherDefault::<WyHash>::default()), sample_u64(count))
        }, |(map, ints)| {
            for int in ints {
                let len = *int >> 3;
                map.insert(*int, len);
            }
        }, criterion::BatchSize::LargeInput);
    })
}

fn bench_wyhash_object(count: usize) -> Box<dyn FnMut(&mut Bencher)> {
    Box::new(move |b: &mut Bencher| {
        b.iter_batched_ref(|| {
            (std::collections::HashSet::with_hasher(BuildHasherDefault::<WyHash>::default()), sample_object(count))
        }, |(set, objs)| {
            for obj in objs {
                set.insert(obj.clone());
            }
        }, criterion::BatchSize::LargeInput);
    })
}
