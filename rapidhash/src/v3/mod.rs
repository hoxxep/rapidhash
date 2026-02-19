//! Portable hashing: rapidhash V3 algorithm.

mod rapid_const;
#[cfg(any(feature = "std", docsrs))]
mod rapid_file;
mod seed;
mod rapid_stream_hasher;

#[doc(inline)]
pub use rapid_const::*;

#[doc(inline)]
#[cfg(any(feature = "std", docsrs))]
pub use rapid_file::*;

#[doc(inline)]
pub use rapid_stream_hasher::*;

#[doc(inline)]
pub use seed::*;

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::util::macros::{compare_to_c, flip_bit_trial};
    use rand::Rng;

    flip_bit_trial!(flip_bit_trial_v3, rapidhash_v3_inline::<true, false, false>);
    flip_bit_trial!(flip_bit_trial_v3_micro, rapidhash_v3_micro_inline::<true, false>);
    flip_bit_trial!(flip_bit_trial_v3_nano, rapidhash_v3_nano_inline::<true, false>);
    compare_to_c!(compare_to_c_v3, rapidhash_v3_inline::<true, false, false>, rapidhash_v3_inline::<true, true, false>, rapidhashcc_v3);
    compare_to_c!(compare_to_c_v3_micro, rapidhash_v3_micro_inline::<true, false>, rapidhash_v3_micro_inline::<true, false>, rapidhashcc_v3_micro);
    compare_to_c!(compare_to_c_v3_nano, rapidhash_v3_nano_inline::<true, false>, rapidhash_v3_nano_inline::<true, false>, rapidhashcc_v3_nano);

    #[test]
    fn with_seed_zero_matches_unseeded_v3() {
        let mut rng = rand::rng();
        for len in 0..=2048 {
            let mut data = std::vec![0; len];
            rng.fill(&mut data[..]);
            assert_eq!(rapidhash_v3_with_seed(&data, 0), rapidhash_v3(&data), "Mismatch on len {len}");
        }
    }

    #[test]
    fn with_seed_matches_seed_cpp_path() {
        let mut rng = rand::rng();
        let mut seed_corpus = std::vec![
            0,
            1,
            2,
            3,
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210,
            u64::MAX,
        ];
        for _ in 0..64 {
            seed_corpus.push(rng.random());
        }

        for len in 0..=512 {
            let mut data = std::vec![0; len];
            rng.fill(&mut data[..]);
            for seed in &seed_corpus {
                let expected = rapidhash_v3_seeded(&data, &RapidSecrets::seed_cpp(*seed));
                let actual = rapidhash_v3_with_seed(&data, *seed);
                assert_eq!(actual, expected, "Mismatch on len {len} seed {seed}");
            }
        }
    }

    /// Compare the main rapidhash version matches micro (80 btyes) and nano (48 bytes) up to
    /// the expected length.
    #[test]
    fn compare_micro_nano_v3() {
        // test zero-length input
        let hash_v3 = rapidhash_v3_inline::<true, false, false>(&[], &DEFAULT_RAPID_SECRETS);
        let hash_micro = rapidhash_v3_micro_inline::<true, false>(&[], &DEFAULT_RAPID_SECRETS);
        let hash_nano = rapidhash_v3_nano_inline::<true, false>(&[], &DEFAULT_RAPID_SECRETS);
        assert_eq!(hash_v3, hash_micro, "Mismatch with micro on zero length input");
        assert_eq!(hash_v3, hash_nano, "Mismatch with nano on zero length input");

        for len in 0..=82 {
            let mut data = std::vec![0; len];
            rand::rng().fill(&mut data[..]);

            for byte in 0..len {
                for bit in 0..8 {
                    let mut data = data.clone();
                    data[byte] ^= 1 << bit;

                    let hash_v3 = rapidhash_v3_inline::<true, false, false>(&data, &DEFAULT_RAPID_SECRETS);
                    let hash_micro = rapidhash_v3_micro_inline::<true, false>(&data, &DEFAULT_RAPID_SECRETS);
                    let hash_nano = rapidhash_v3_nano_inline::<true, false>(&data, &DEFAULT_RAPID_SECRETS);

                    if len <= 80 {
                        assert_eq!(hash_v3, hash_micro, "Mismatch with mico on input {} byte {} bit {}", len, byte, bit);
                    } else {
                        assert_ne!(hash_v3, hash_micro, "Micro should mismatch on input {} byte {} bit {}", len, byte, bit);
                    }

                    if len <= 48 {
                        assert_eq!(hash_v3, hash_nano, "Mismatch with nano on input {} byte {} bit {}", len, byte, bit);
                    } else {
                        assert_ne!(hash_v3, hash_nano, "Nano should mismatch on input {} byte {} bit {}", len, byte, bit);
                    }
                }
            }
        }
    }
}
