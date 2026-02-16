//! Benchmarking rapidhash v3 interfaces: raw function, file hasher, and stream hasher.
//!
//! Compares the three v3 hashing interfaces at various input sizes and chunk granularities.

use std::hint::black_box;
use std::io::Cursor;
use criterion::{BenchmarkGroup, Criterion, Throughput};
use criterion::measurement::WallTime;
use rand::Rng;

const INPUT_SIZES: &[usize] = &[50, 500, 5_000, 50_000, 5_000_000];
const CHUNK_SIZES: &[usize] = &[16, 128, 1024, 6000];

fn make_data(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    rand::rng().fill(buf.as_mut_slice());
    buf
}

fn bench_input_size(group: &mut BenchmarkGroup<'_, WallTime>, input_size: usize) {
    let secrets = rapidhash::v3::RapidSecrets::seed(0);

    group.throughput(Throughput::Bytes(input_size as u64));

    // --- rapidhash v3 raw function (baseline) ---
    group.bench_function(&format!("v3_raw/input_{input_size}"), |b| {
        b.iter_batched_ref(
            || make_data(input_size),
            |data| black_box(rapidhash::v3::rapidhash_v3_seeded(black_box(data), &secrets)),
            criterion::BatchSize::SmallInput,
        );
    });

    // --- rapidhash v3 file hasher: single write ---
    group.bench_function(&format!("v3_file/input_{input_size}/chunk_{input_size}"), |b| {
        b.iter_batched_ref(
            || make_data(input_size),
            |data| {
                let cursor = Cursor::new(black_box(data.as_slice()));
                black_box(rapidhash::v3::rapidhash_v3_file_seeded(cursor, &secrets).unwrap())
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // --- rapidhash v3 stream hasher: single write ---
    group.bench_function(&format!("v3_stream/input_{input_size}/chunk_{input_size}"), |b| {
        let mut hasher = rapidhash::v3::RapidStreamHasherV3::new(&secrets);
        b.iter_batched_ref(
            || make_data(input_size),
            |data| {
                hasher.reset();
                hasher.write(black_box(data));
                black_box(hasher.finish())
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // --- chunked writes for file and stream hashers ---
    for &chunk_size in CHUNK_SIZES {
        if chunk_size >= input_size {
            continue;
        }

        // file hasher with chunked reads (simulated via ChunkedCursor)
        group.bench_function(&format!("v3_file/input_{input_size}/chunk_{chunk_size}"), |b| {
            b.iter_batched_ref(
                || make_data(input_size),
                |data| {
                    let cursor = ChunkedCursor::new(black_box(data.as_slice()), black_box(chunk_size));
                    black_box(rapidhash::v3::rapidhash_v3_file_seeded(cursor, &secrets).unwrap())
                },
                criterion::BatchSize::SmallInput,
            );
        });

        // stream hasher with chunked writes
        group.bench_function(&format!("v3_stream/input_{input_size}/chunk_{chunk_size}"), |b| {
            let mut hasher = rapidhash::v3::RapidStreamHasherV3::new(&secrets);
            b.iter_batched_ref(
                || make_data(input_size),
                |data| {
                    hasher.reset();
                    let data = black_box(data.as_slice());
                    for chunk in data.chunks(black_box(chunk_size)) {
                        hasher.write(chunk);
                    }
                    black_box(hasher.finish())
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming");
    group.warm_up_time(std::time::Duration::from_millis(250));
    group.measurement_time(std::time::Duration::from_millis(2000));
    group.sample_size(100);

    for &input_size in INPUT_SIZES {
        bench_input_size(&mut group, input_size);
    }

    group.finish();
}

/// A `Read` adapter that yields at most `max_chunk` bytes per `read()` call,
/// simulating small block I/O for the file hasher benchmarks.
struct ChunkedCursor<'a> {
    data: &'a [u8],
    pos: usize,
    max_chunk: usize,
}

impl<'a> ChunkedCursor<'a> {
    fn new(data: &'a [u8], max_chunk: usize) -> Self {
        Self { data, pos: 0, max_chunk }
    }
}

impl<'a> std::io::Read for ChunkedCursor<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.data.len() - self.pos;
        let n = remaining.min(self.max_chunk).min(buf.len());
        buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}
