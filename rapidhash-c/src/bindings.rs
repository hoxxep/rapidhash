macro_rules! bindings {
    ($cpp:ident, $rust:ident, $lib:literal, $test:ident) => {
        #[link(name = $lib, kind = "static")]
        extern "C" {
            fn $cpp(
                key: *const core::ffi::c_void,
                len: libc::size_t,
                seed: u64,
            ) -> u64;
        }

        #[inline]
        pub fn $rust(key: &[u8], seed: u64) -> u64 {
            unsafe { $cpp(key.as_ptr() as *const core::ffi::c_void, key.len(), seed) }
        }

        #[cfg(test)]
        mod $test {
            use super::*;

            /// Ensure that changing a single bit flips at least 10 bits in the resulting hash, and on
            /// average flips half of the bits.
            ///
            /// These tests are not deterministic, but should fail with a very low probability.
            #[test]
            fn flip_bit_trial() {
                use rand::{Rng};
                let mut rng: rapidrand::RapidRng = rand::make_rng();

                let mut flips = std::vec![];

                for len in 1..=512 {
                    let mut data = std::vec![0; len];
                    rng.fill_bytes(&mut data[..]);

                    let hash = $rust(&data, 0);
                    for byte in 0..len {
                        for bit in 0..8 {
                            let mut data = data.clone();
                            data[byte] ^= 1 << bit;
                            let new_hash = $rust(&data, 0);
                            assert_ne!(hash, new_hash, "Flipping byte {} bit {} did not change hash for input len {}", byte, bit, len);
                            let xor = hash ^ new_hash;
                            let flipped = xor.count_ones() as u64;
                            assert!(xor.count_ones() >= 8, "Flipping bit {byte}:{bit} changed only {flipped} bits");

                            flips.push(flipped);
                        }
                    }
                }

                let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
                assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {average}, expected: 32.0");
            }
        }
    };
}

bindings!(rapidhash_v1_extern, rapidhashcc_v1, "rapidhash_v1", tests_v1);
bindings!(rapidhash_v2_extern, rapidhashcc_v2, "rapidhash_v2", tests_v2);
bindings!(rapidhash_v2_1_extern, rapidhashcc_v2_1, "rapidhash_v2_1", tests_v2_1);
bindings!(rapidhash_v2_2_extern, rapidhashcc_v2_2, "rapidhash_v2_2", tests_v2_2);
bindings!(rapidhash_v3_extern, rapidhashcc_v3, "rapidhash_v3", tests_v3);
bindings!(rapidhash_v3_micro_extern, rapidhashcc_v3_micro, "rapidhash_v3", tests_v3_micro);
bindings!(rapidhash_v3_nano_extern, rapidhashcc_v3_nano, "rapidhash_v3", tests_v3_nano);
bindings!(rapidhash_rs_extern, rapidhashcc_rs, "rapidhash_rs", tests_rs);

#[cfg(test)]
mod tests_verification {
    use super::*;

    /// Used to generate the SMHasher3 selftest expected values.
    #[test]
    fn verification_rs() {
        let inputs: [(u64, &str); 8] = [
            (0x0894342f79b4e6d9, ""),
            (0x8b58663657733d01, "a"),
            (0xbef2833301a85a24, "abc"),
            (0x3942bb61ee99487d, "message digest"),
            (0x84ceed9776a410a4, "abcdefghijklmnopqrstuvwxyz"),
            (0x66afe1ce46f2a1fe, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            (0xcd2631a72e5e12eb, "12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
            (0x56e9951f176b61c4, "vlong123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"),
        ];

        for (i, (expected, input)) in inputs.iter().enumerate() {
            // let seed = preseed_v3(0);
            let result = rapidhashcc_rs(input.as_bytes(), i as u64);
            let prefix: String = input.chars().take(16).collect();
            assert_eq!(
                result, *expected,
                "Got 0x{} for input {} of '{}'",
                hex::encode(result.to_be_bytes()),
                i,
                prefix,
            );
        }
    }
}
