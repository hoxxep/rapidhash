use std::hash::{BuildHasher, BuildHasherDefault};
use criterion::{Bencher, Criterion, Throughput};
use rand::distr::{Alphanumeric, SampleString};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::Distribution;
use wyhash::WyHash;

pub fn bench(c: &mut Criterion) {
    let groups: &[(
        &str,
        Box<dyn Fn() -> Box<dyn FnMut(&mut Bencher)>>,
    )] = &[
        ("hash/rapidhash", Box::new(bench_rapidhash)),
        ("hash/rapidhash_raw", Box::new(bench_rapidhash_raw)),
        ("hash/rapidhash_cc_v1", Box::new(bench_rapidhash_cc_v1)),
        ("hash/rapidhash_cc_v2", Box::new(bench_rapidhash_cc_v2)),
        ("hash/rapidhash_cc_v3", Box::new(bench_rapidhash_cc_v3)),
        ("hash/default", Box::new(bench_default)),
        ("hash/fxhash", Box::new(bench_fxhash)),
        ("hash/gxhash", Box::new(bench_gxhash)),
        ("hash/wyhash", Box::new(bench_wyhash)),
        ("hash/foldhash", Box::new(bench_foldhash)),
    ];

    for (name, bench_fn) in groups {
        let mut group = c.benchmark_group(name.to_string());
        group.throughput(Throughput::Elements(1u64));
        group.bench_function("emails", bench_fn());
    }
}

fn sample_emails(count: usize) -> Vec<String> {
    let mut rng = rand::rng();

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

            Alphanumeric.sample_string(&mut rng, length)
        })
        .collect()
}

macro_rules! bench_hash_emails {
    ($func:ident, $hasher:expr) => {
        fn $func () -> Box<dyn FnMut(&mut Bencher)> {
            use std::hash::Hasher;

            Box::new(move |b: &mut Bencher| {
                b.iter_batched_ref(|| {
                    let builder = $hasher;
                    let sample = sample_emails(1)[0].clone();
                    (builder, sample)
                }, |(builder, sample)| {
                    // builder.hash_one(sample.as_bytes())
                    let mut hasher = builder.build_hasher();
                    hasher.write(&sample.as_bytes());
                    hasher.finish()
                }, criterion::BatchSize::SmallInput);
            })
        }
    };
}

macro_rules! bench_hash_emails_raw {
    ($func:ident, $hasher:path) => {
        fn $func () -> Box<dyn FnMut(&mut Bencher)> {
            Box::new(move |b: &mut Bencher| {
                b.iter_batched_ref(|| {
                    let sample = sample_emails(1)[0].clone();
                    (sample,)
                }, |(sample,)| {
                    $hasher (sample.as_bytes(), 0)
                }, criterion::BatchSize::SmallInput);
            })
        }
    };
}

fn v3_bench(data: &[u8], seed: u64) -> u64 {
    let secrets = rapidhash::v3::RapidSecrets::seed_cpp(seed);
    rapidhash::v3::rapidhash_v3_seeded(data, &secrets)
}

bench_hash_emails!(bench_rapidhash, rapidhash::quality::RapidBuildHasher::default());
bench_hash_emails_raw!(bench_rapidhash_cc_v1, rapidhash_c::rapidhashcc_v1);
bench_hash_emails_raw!(bench_rapidhash_cc_v2, rapidhash_c::rapidhashcc_v2);
bench_hash_emails_raw!(bench_rapidhash_cc_v3, rapidhash_c::rapidhashcc_v3);
bench_hash_emails_raw!(bench_rapidhash_raw, v3_bench);
bench_hash_emails!(bench_default, std::hash::RandomState::default());
bench_hash_emails!(bench_fxhash, fxhash::FxBuildHasher::default());
bench_hash_emails!(bench_gxhash, gxhash::GxBuildHasher::default());
bench_hash_emails!(bench_wyhash, BuildHasherDefault::<WyHash>::default());
bench_hash_emails!(bench_foldhash, foldhash::quality::RandomState::default());
