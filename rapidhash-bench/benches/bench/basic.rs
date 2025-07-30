//! Benchmarking hashers against integers and byte slices of various lengths.

use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::hint::black_box;
use criterion::{BenchmarkGroup, Criterion, Throughput};
use criterion::measurement::WallTime;
use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

fn profile_bytes<H: BuildHasher + Default>(
    bytes_len: usize,
    group: &mut BenchmarkGroup<'_, WallTime>,
) {
    let name = format!("str_{bytes_len}");
    let build_hasher = H::default();

    if bytes_len > 1024 * 1024 {
        group.sample_size(10);
    } else {
        group.sample_size(100);
    }
    group.throughput(Throughput::Bytes(bytes_len as u64));
    group.bench_function(&name, |b| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; bytes_len];
            rand::rng().fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            // hash_one seems to cause significant overhead for some hashers, likely related to
            // inlining because of the amount of indirection it does.
            // black_box(build_hasher.hash_one(black_box(bytes)))

            let mut hasher = build_hasher.build_hasher();
            hasher.write(black_box(bytes));
            black_box(hasher.finish())
        }, criterion::BatchSize::SmallInput);
    });
}

fn profile_int<H: BuildHasher + Default, I: Hash>(
    int_name: &str,
    group: &mut BenchmarkGroup<'_, WallTime>,
)
where
    StandardUniform: Distribution<I>,
{
    let name = format!("{int_name}");
    let build_hasher = H::default();

    group.sample_size(100);
    group.throughput(Throughput::Elements(1));
    group.bench_function(&name, |b| {
        b.iter_batched(
            || rand::random::<I>(),
            |value| {
                black_box(build_hasher.hash_one(black_box(value)))
            },
            criterion::BatchSize::SmallInput);
    });
}

fn bench_group<H: BuildHasher + Default>(c: &mut Criterion, group_name: &str) {
    let mut group = c.benchmark_group(group_name.to_string());
    let sizes = [2usize, 8, 16, 25, 50, 64, 80, 160, 256, 350, 1024, 4096, 65536, 1024 * 1024 * 500];
    for &size in &sizes {
        profile_bytes::<H>(size, &mut group);
    }
    profile_int::<H, u8>("u8", &mut group);
    profile_int::<H, u16>("u16", &mut group);
    profile_int::<H, u32>("u32", &mut group);
    profile_int::<H, u64>("u64", &mut group);
    profile_int::<H, u128>("u128", &mut group);
}

fn profile_bytes_raw<H: Fn(&[u8], u64) -> u64>(
    hash: &H,
    bytes_len: usize,
    group: &mut BenchmarkGroup<'_, WallTime>,
) {
    let name = format!("str_{bytes_len}");

    if bytes_len > 1024 * 1024 {
        group.sample_size(10);
    } else {
        group.sample_size(100);
    }
    group.throughput(Throughput::Bytes(bytes_len as u64));
    group.bench_function(&name, |b| {
        b.iter_batched_ref(|| {
            let mut slice = vec![0u8; bytes_len];
            rand::rng().fill(slice.as_mut_slice());
            slice
        }, |bytes| {
            black_box(hash(black_box(bytes), 0xbdd89aa982704029))  // using rapidhash V1 seed
        }, criterion::BatchSize::SmallInput);
    });
}

fn bench_group_raw<H: Fn(&[u8], u64) -> u64>(c: &mut Criterion, group_name: &str, hash: &H) {
    let mut group = c.benchmark_group(group_name.to_string());
    let sizes = [2usize, 8, 16, 25, 50, 64, 80, 160, 256, 350, 1024, 4096, 65536, 1024 * 1024 * 500];
    for &size in &sizes {
        profile_bytes_raw(hash, size, &mut group);
    }
}

pub fn bench(c: &mut Criterion) {
    bench_group::<rapidhash::fast::RandomState>(c, "hash/rapidhash-f");
    bench_group::<rapidhash::quality::RandomState>(c, "hash/rapidhash-q");

    bench_group::<foldhash::fast::RandomState>(c, "hash/foldhash-f");
    bench_group::<foldhash::quality::RandomState>(c, "hash/foldhash-q");

    bench_group::<std::hash::RandomState>(c, "hash/default");
    bench_group::<fxhash::FxBuildHasher>(c, "hash/fxhash");
    bench_group::<gxhash::GxBuildHasher>(c, "hash/gxhash");
    bench_group::<ahash::RandomState>(c, "hash/ahash");
    bench_group::<t1ha::T1haBuildHasher>(c, "hash/t1ha");
    bench_group::<wyhash::WyHasherBuilder>(c, "hash/wyhash");
    bench_group::<BuildHasherDefault<twox_hash::XxHash64>>(c, "hash/xxhash64");
    bench_group::<BuildHasherDefault<twox_hash::XxHash3_64>>(c, "hash/xxhash3_64");
    bench_group::<metrohash::MetroBuildHasher>(c, "hash/metrohash");
    bench_group::<BuildHasherDefault<seahash::SeaHasher>>(c, "hash/seahash");
    bench_group::<BuildHasherDefault<farmhash::FarmHasher>>(c, "hash/farmhash");
    bench_group::<highway::HighwayBuildHasher>(c, "hash/highwayhash");
    bench_group::<rustc_hash::FxBuildHasher>(c, "hash/rustc-hash");

    bench_group_raw(c, "hash/rapidhash_raw", &v3_bench);
    bench_group_raw(c, "hash/rapidhash_cc_v1", &rapidhash_c::rapidhashcc_v1);
    bench_group_raw(c, "hash/rapidhash_cc_v2", &rapidhash_c::rapidhashcc_v2);
    bench_group_raw(c, "hash/rapidhash_cc_v2_1", &rapidhash_c::rapidhashcc_v2_1);
    bench_group_raw(c, "hash/rapidhash_cc_v2_2", &rapidhash_c::rapidhashcc_v2_2);
    bench_group_raw(c, "hash/rapidhash_cc_v3", &rapidhash_c::rapidhashcc_v3);
    bench_group_raw(c, "hash/rapidhash_cc_rs", &rapidhash_c::rapidhashcc_rs);
    bench_group_raw(c, "hash/wyhash_raw", &wyhash::wyhash);
}

fn v3_bench(data: &[u8], seed: u64) -> u64 {
    let secrets = rapidhash::v3::RapidSecrets::seed_cpp(seed);
    rapidhash::v3::rapidhash_v3_seeded(data, &secrets)
}
