mod bindings;

pub use bindings::*;

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use super::*;

    /// This test demonstrates seed-independent hash collisions in rapidhash v3 when using the
    /// default secrets. This is why we randomize the secrets in the rust crate.
    #[test]
    fn seed_independent_hash_collisions() {
        const SECRET1: u64 = 0x8bb84b93962eacc9;
        const EXPECTED_HASH: u64 = 18446744073709551615;

        fn random_slice() -> Vec<u8> {
            // generate a random 32-byte input
            let mut data = vec![0; 32];
            let mut rng = rapidrand::RapidRng::from_rng(&mut rand::rng());
            rng.fill_bytes(data.as_mut_slice());

            // set penultimate 8 bytes to secret[1], XORed with the input length 32
            let offset = data.len() - 16;
            let a = &mut data[offset .. offset + 8];
            a.copy_from_slice(&(SECRET1 ^ 32).to_le_bytes());

            data
        }

        for _ in 0..10 {
            let hash = rapidhashcc_v3(&random_slice(), rand::random::<u64>());
            assert_eq!(hash, EXPECTED_HASH, "Expected seed-independent hash collision.");
        }
    }
}
