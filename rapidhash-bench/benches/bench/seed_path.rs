//! Benchmark seeded v3 call paths:
//! - pointer-based seeded API: `rapidhash_v3_seeded(data, &RapidSecrets)`
//! - seed-by-value API: `rapidhash_v3_with_seed(data, seed)`

use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, Criterion, Throughput};
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};
use std::hint::black_box;

const RUNTIME_SEED: u64 = 0x1234_5678_9abc_def0;
const LOOKUP_SEED: u64 = 0x0f1e_2d3c_4b5a_6978;
const INPUT_POOL: usize = 1024;
const LOOKUP_MAP_SIZE: usize = 10_000;
const LOOKUP_QUERY_COUNT: usize = 20_000;

static LOOKUP_SECRETS: rapidhash::v3::RapidSecrets =
    rapidhash::v3::RapidSecrets::seed_cpp(LOOKUP_SEED);

#[derive(Clone, Eq, PartialEq)]
struct BytesKey(Box<[u8]>);

impl Hash for BytesKey {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

#[derive(Clone, Copy)]
struct SeededPointerPath {
    secrets: rapidhash::v3::RapidSecrets,
}

#[derive(Clone, Copy)]
struct SeedByValuePath {
    seed: u64,
}

#[inline(never)]
fn hash_seeded_pointer_from_state(data: &[u8], state: &SeededPointerPath) -> u64 {
    rapidhash::v3::rapidhash_v3_seeded(data, &state.secrets)
}

#[inline(never)]
fn hash_with_seed_from_state(data: &[u8], state: &SeedByValuePath) -> u64 {
    rapidhash::v3::rapidhash_v3_with_seed(data, state.seed)
}

#[derive(Clone, Copy)]
struct V3SeededPointerBuildHasher {
    secrets: rapidhash::v3::RapidSecrets,
}

#[derive(Clone, Copy)]
struct V3SeededPointerHasher {
    secrets: rapidhash::v3::RapidSecrets,
    hash: u64,
}

impl BuildHasher for V3SeededPointerBuildHasher {
    type Hasher = V3SeededPointerHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        V3SeededPointerHasher {
            secrets: self.secrets,
            hash: 0,
        }
    }
}

impl Hasher for V3SeededPointerHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = rapidhash::v3::rapidhash_v3_seeded(bytes, &self.secrets);
    }
}

#[derive(Clone, Copy)]
struct V3WithSeedBuildHasher {
    seed: u64,
}

#[derive(Clone, Copy)]
struct V3WithSeedHasher {
    seed: u64,
    hash: u64,
}

impl BuildHasher for V3WithSeedBuildHasher {
    type Hasher = V3WithSeedHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        V3WithSeedHasher {
            seed: self.seed,
            hash: 0,
        }
    }
}

impl Hasher for V3WithSeedHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = rapidhash::v3::rapidhash_v3_with_seed(bytes, self.seed);
    }
}

#[derive(Clone, Copy, Default)]
struct V3UnseededBuildHasher;

#[derive(Clone, Copy, Default)]
struct V3UnseededHasher {
    hash: u64,
}

impl BuildHasher for V3UnseededBuildHasher {
    type Hasher = V3UnseededHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        V3UnseededHasher { hash: 0 }
    }
}

impl Hasher for V3UnseededHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = rapidhash::v3::rapidhash_v3(bytes);
    }
}

fn sample_inputs(len: usize) -> Vec<Box<[u8]>> {
    debug_assert!(INPUT_POOL.is_power_of_two());
    let mut rng = rand::rngs::StdRng::seed_from_u64(0x9f2d_6d10_733a_12bd ^ len as u64);
    (0..INPUT_POOL)
        .map(|_| {
            let mut input = vec![0u8; len];
            rng.fill(input.as_mut_slice());
            input.into_boxed_slice()
        })
        .collect()
}

fn sample_lookup_dataset() -> (Vec<BytesKey>, Vec<BytesKey>) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xc4d3_22f1_91aa_18e3);
    let map_keys: Vec<BytesKey> = (0..LOOKUP_MAP_SIZE)
        .map(|_| {
            let len = rng.random_range(2..=8);
            let mut data = vec![0u8; len];
            rng.fill(data.as_mut_slice());
            BytesKey(data.into_boxed_slice())
        })
        .collect();

    let hit_queries: Vec<BytesKey> = (0..LOOKUP_QUERY_COUNT)
        .map(|_| map_keys[rng.random_range(0..map_keys.len())].clone())
        .collect();

    (map_keys, hit_queries)
}

fn bench_hash_path<F>(group: &mut BenchmarkGroup<'_, WallTime>, label: &str, len: usize, hash: F)
where
    F: Fn(&[u8]) -> u64 + Copy,
{
    let name = format!("{label}/len_{len}");
    let inputs = sample_inputs(len);
    let mut idx: usize = 0;
    group.throughput(Throughput::Elements(1));
    group.bench_function(&name, move |b| {
        b.iter(|| {
            idx = idx.wrapping_add(1);
            let input = &inputs[idx & (INPUT_POOL - 1)];
            black_box(hash(black_box(input.as_ref())))
        });
    });
}

fn build_map<B: BuildHasher>(build_hasher: B, map_keys: &[BytesKey]) -> HashMap<BytesKey, u32, B> {
    let mut map = HashMap::with_capacity_and_hasher(map_keys.len(), build_hasher);
    for (idx, key) in map_keys.iter().cloned().enumerate() {
        map.insert(key, idx as u32);
    }
    map
}

fn bench_lookup_path<B: BuildHasher + Clone>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    build_hasher: B,
    map_keys: &[BytesKey],
    hit_queries: &[BytesKey],
) {
    let map = build_map(build_hasher, map_keys);

    group.throughput(Throughput::Elements(hit_queries.len() as u64));
    group.bench_function(format!("{label}/lookup_hit"), |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for key in hit_queries {
                if let Some(v) = map.get(black_box(key)) {
                    sum = sum.wrapping_add(*v as u64);
                }
            }
            black_box(sum)
        });
    });
}

pub fn bench(c: &mut Criterion) {
    let pointer_path = SeededPointerPath {
        secrets: rapidhash::v3::RapidSecrets::seed_cpp(RUNTIME_SEED),
    };
    let by_value_path = SeedByValuePath { seed: RUNTIME_SEED };

    let mut short_group = c.benchmark_group("seed-path/short");
    short_group.warm_up_time(std::time::Duration::from_millis(250));
    short_group.measurement_time(std::time::Duration::from_millis(1000));
    short_group.sample_size(100);

    for len in 1..=16 {
        bench_hash_path(
            &mut short_group,
            "v3_unseeded",
            len,
            rapidhash::v3::rapidhash_v3,
        );
        bench_hash_path(&mut short_group, "v3_with_seed_zero", len, |data| {
            rapidhash::v3::rapidhash_v3_with_seed(data, 0)
        });
        bench_hash_path(&mut short_group, "v3_with_seed_const", len, |data| {
            rapidhash::v3::rapidhash_v3_with_seed(data, RUNTIME_SEED)
        });
        bench_hash_path(&mut short_group, "v3_with_seed_runtime", len, |data| {
            hash_with_seed_from_state(data, black_box(&by_value_path))
        });
        bench_hash_path(&mut short_group, "v3_seeded_pointer", len, |data| {
            hash_seeded_pointer_from_state(data, black_box(&pointer_path))
        });
    }
    short_group.finish();

    let mut long_group = c.benchmark_group("seed-path/long");
    long_group.warm_up_time(std::time::Duration::from_millis(250));
    long_group.measurement_time(std::time::Duration::from_millis(1500));
    long_group.sample_size(100);
    for len in [64usize, 256, 1024] {
        bench_hash_path(
            &mut long_group,
            "v3_unseeded",
            len,
            rapidhash::v3::rapidhash_v3,
        );
        bench_hash_path(&mut long_group, "v3_with_seed_zero", len, |data| {
            rapidhash::v3::rapidhash_v3_with_seed(data, 0)
        });
        bench_hash_path(&mut long_group, "v3_with_seed_const", len, |data| {
            rapidhash::v3::rapidhash_v3_with_seed(data, RUNTIME_SEED)
        });
        bench_hash_path(&mut long_group, "v3_with_seed_runtime", len, |data| {
            hash_with_seed_from_state(data, black_box(&by_value_path))
        });
        bench_hash_path(&mut long_group, "v3_seeded_pointer", len, |data| {
            hash_seeded_pointer_from_state(data, black_box(&pointer_path))
        });
    }
    long_group.finish();

    let (map_keys, hit_queries) = sample_lookup_dataset();
    let mut lookup_group = c.benchmark_group("seed-path/lookup");
    lookup_group.warm_up_time(std::time::Duration::from_millis(250));
    lookup_group.measurement_time(std::time::Duration::from_millis(1500));
    lookup_group.sample_size(60);
    lookup_group.sampling_mode(criterion::SamplingMode::Flat);

    bench_lookup_path(
        &mut lookup_group,
        "v3_unseeded",
        V3UnseededBuildHasher,
        &map_keys,
        &hit_queries,
    );
    bench_lookup_path(
        &mut lookup_group,
        "v3_with_seed",
        V3WithSeedBuildHasher { seed: LOOKUP_SEED },
        &map_keys,
        &hit_queries,
    );
    bench_lookup_path(
        &mut lookup_group,
        "v3_seeded_pointer",
        V3SeededPointerBuildHasher {
            secrets: LOOKUP_SECRETS,
        },
        &map_keys,
        &hit_queries,
    );
    lookup_group.finish();
}
