
#[cfg(test)]
mod tests {
    use std::hash::{BuildHasher, Hasher};

    #[cfg(test)]
    mod tests {
        #[cfg(feature = "bench")]
        #[test]
        fn test_hashmap_size() {
            // only enable when std is available
            assert!(core::mem::size_of::<rapidhash::RapidHashMap<u32, u32>>() <= 40);
            assert!(core::mem::size_of::<foldhash::HashMap<u32, u32>>() <= 40);
        }

        #[cfg(feature = "bench")]
        #[test]
        fn test_hasher_size() {
            // only enable when std is available
            assert!(core::mem::size_of::<rapidhash::fast::RapidHasher>() <= 48);
            assert!(core::mem::size_of::<foldhash::fast::FoldHasher>() <= 48);
        }
    }


    /// Helper method for [flip_bit_trial_streaming]. Hashes a byte stream in u8 chunks.
    fn streaming_hash<T: BuildHasher>(hasher: &T, data: &[u8]) -> u64 {
        let mut hasher = hasher.build_hasher();
        for byte in data {
            hasher.write_u8(*byte);
        }
        hasher.finish()
    }

    #[test]
    fn foldhash_flip_bit_trial_streaming() {
        use rand::Rng;

        let hash_builder = foldhash::fast::FixedState::default();
        let mut flips = std::vec![];

        for len in 1..=512 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            let hash = streaming_hash(&hash_builder, &data);
            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;

                    // check that the hash changed
                    let new_hash = streaming_hash(&hash_builder, &data);
                    assert_ne!(hash, new_hash, "Flipping bit {byte}:{bit} for input len {len} did not change hash");

                    // track how many bits were flipped
                    let xor = hash ^ new_hash;
                    let flipped = xor.count_ones() as u64;
                    assert!(xor.count_ones() >= 8, "Flipping bit {byte}:{bit} for input len {len} changed only {flipped} bits");
                    flips.push(flipped);
                }
            }
        }

        // check that on average half of the bits were flipped
        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
    }
}
