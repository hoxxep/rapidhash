#!/usr/bin/python3

import glob
from xxsubtype import bench

import cbor2
from matplotlib import pyplot as plt

def main():
    draw_hash()
    draw_map()


def draw_hash():
    hash_settings = [
        ("rapidhash", "b"),
        ("default", "k"),
        ("fxhash", "r"),
        ("gxhash", "m"),
        ("wyhash", "c"),
        ("ahash", "0.8"),
        ("t1ha", "0.8"),
        ("metrohash", "0.8"),
        ("seahash", "0.8"),
        ("xxhash", "0.8"),
    ]

    hash_names = [hash_function for hash_function, _ in hash_settings]

    sizes = [2, 8, 16, 64, 256, 1024, 4096]

    latency_data = []
    throughput_data = []
    latency_data_u64 = []
    throughput_data_u64 = []
    for (hash_function, _) in hash_settings:
        latency_row = []
        throughput_row = []

        for size in sizes:
            latency, throughput = load_latest_measurement_file("hash", hash_function, f"str_{size}")
            latency_row.append(latency)
            throughput_row.append(throughput)

        latency_data.append(latency_row)
        throughput_data.append(throughput_row)

        latency, throughput = load_latest_measurement_file("hash", hash_function, "u64")
        latency_data_u64.append(latency)
        throughput_data_u64.append(throughput)

    fig, axs = plt.subplots(2, 2, figsize=(12, 8), dpi=300)

    for i, (hash_function, color) in reversed(list(enumerate(hash_settings))):
        axs[0, 0].plot(sizes, latency_data[i], label=hash_function, color=color)
        axs[0, 1].plot(sizes, throughput_data[i], label=hash_function, color=color)

        # Annotate the end of each line
        axs[0, 0].annotate(hash_function, (sizes[-1], latency_data[i][-1]), color=color,
                           xytext=(25, 0), textcoords='offset points', ha='left', va='center')
        axs[0, 1].annotate(hash_function, (sizes[-1], throughput_data[i][-1]), color=color,
                           xytext=(25, 0), textcoords='offset points', ha='left', va='center')

    for i, (hash_function, color) in enumerate(hash_settings):
        print(hash_function, i, latency_data_u64[i], throughput_data_u64[i])
        axs[1, 0].bar(hash_function, latency_data_u64[i], color=color, zorder=3)
        axs[1, 1].bar(hash_function, throughput_data_u64[i], color=color, zorder=3)

    axs[0, 0].set_title("Latency (byte stream)")
    axs[0, 0].set_xlabel("Input size (bytes)")
    axs[0, 0].set_ylabel("Latency (ns)")
    axs[0, 0].set_xscale("log")
    axs[0, 0].set_yscale("log")
    axs[0, 0].set_xticks(sizes)
    axs[0, 0].set_xticklabels(sizes)

    axs[0, 1].set_title("Throughput (byte stream)")
    axs[0, 1].set_xlabel("Input size (bytes)")
    axs[0, 1].set_ylabel("Throughput (GB/s)")
    axs[0, 1].set_xscale("log")
    axs[0, 1].set_yscale("log")
    axs[0, 1].set_xticks(sizes)
    axs[0, 1].set_xticklabels(sizes)

    axs[1, 0].set_title("Latency (u64 optimised)")
    axs[1, 0].set_ylabel("Latency (ns)")
    axs[1, 0].set_xticks(range(len(hash_names)))
    axs[1, 0].set_xticklabels(hash_names, rotation=45, ha="right")
    axs[1, 0].grid(True, zorder=0, color="gainsboro")

    axs[1, 1].set_title("Throughput (u64 optimised)")
    axs[1, 1].set_ylabel("Throughput (M Items/s)")
    axs[1, 1].set_xticks(range(len(hash_names)))
    axs[1, 1].set_xticklabels(hash_names, rotation=45, ha="right")
    axs[1, 1].grid(True, zorder=0, color="gainsboro")

    plt.tight_layout()
    plt.savefig("bench_hash.svg")

def draw_map():
    hash_settings = [
        ("rapidhash_inline", "b"),
        ("default", "k"),
        ("fxhash", "r"),
        ("gxhash", "m"),
        ("wyhash", "c"),
    ]

    hash_names = [hash_function.replace("_inline", "") for hash_function, _ in hash_settings]
    insert_benchmarks = [
        ("10000_emails", "emails"),
        ("450000_words", "words"),
        ("100000_u64", "u64"),
        ("10000_struct", "structs"),
    ]

    throughput_data = []
    for (hash_function, _) in hash_settings:
        throughput_row = []

        for (benchmark, _) in insert_benchmarks:
            _, throughput = load_latest_measurement_file("map", hash_function, benchmark)
            throughput_row.append(throughput)
        throughput_data.append(throughput_row)

    fig, axs = plt.subplots(2, 2, figsize=(12, 8), dpi=300)

    for i, (hash_function, color) in enumerate(hash_settings):
        axs[0, 0].bar(hash_function, throughput_data[i][0], color=color, zorder=3)
        axs[0, 1].bar(hash_function, throughput_data[i][1], color=color, zorder=3)
        axs[1, 0].bar(hash_function, throughput_data[i][2], color=color, zorder=3)
        axs[1, 1].bar(hash_function, throughput_data[i][3], color=color, zorder=3)

    for i, (_, benchmark) in enumerate(insert_benchmarks):
        x = int(i / 2)
        y = int(i % 2)
        assert 0 <= x <= 1
        assert 0 <= y <= 1
        print(i, x, y, benchmark)

        axs[x, y].set_title(f"Throughput ({benchmark})")
        axs[x, y].set_ylabel("Throughput (M Items/s)")
        axs[x, y].set_xticks(range(len(hash_names)))
        axs[x, y].set_xticklabels(hash_names, rotation=45, ha="right")
        axs[x, y].grid(True, zorder=0, color="gainsboro")

    plt.tight_layout()
    plt.savefig("bench_insert.svg")


def load_latest_measurement_file(group: str, hash_function: str, bench: str) -> (float, float):
    measurements = glob.glob(f"../target/criterion/data/main/{group}_{hash_function}/{bench}/measurement*")
    measurements.sort()
    assert len(measurements) > 0, f"No measurements found for {hash_function} {bench}"
    measurement_file = measurements[-1]

    with open(measurement_file, "rb") as f:
        data = cbor2.load(f)
        # print(data)
        latency = data["estimates"]["mean"]["point_estimate"]
        throughput_var = data["throughput"]

        if "Bytes" in throughput_var:
            size = throughput_var["Bytes"]
            throughput = ((1_000_000_000 / latency) * size) / 1_000_000_000
        else:
            size = throughput_var["Elements"] or 450000
            throughput = ((1_000_000_000 / latency) * size) / 1_000_000
        # print(hash_function, bench, size, latency, throughput)

    return latency, throughput


if __name__ == "__main__":
    main()
