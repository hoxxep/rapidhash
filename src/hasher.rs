use core::hash::Hasher;
use crate::rapid::{rapid_mix, rapidhash_core, rapidhash_finish, RAPID_SECRET, RAPID_SEED};

/// A [Hasher] trait compatible hasher that uses the [rapidhash](https://github.com/Nicoshev/rapidhash) algorithm.
pub struct RapidHasher {
    seed: u64,
    a: u64,
    b: u64,
    size: u64,
}

impl RapidHasher {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            a: 0,
            b: 0,
            size: 0,
        }
    }
}

impl Default for RapidHasher {
    #[inline]
    fn default() -> Self {
        Self::new(RAPID_SEED)
    }
}

impl Hasher for RapidHasher {
    #[inline]
    fn finish(&self) -> u64 {
        rapidhash_finish(self.a, self.b, self.size)
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // FUTURE: wyhash processes the bytes as u64::MAX chunks in case chunk.len() > usize.
        const _: () = assert!(usize::MAX as u128 <= u64::MAX as u128, "usize is larger than u64. Please raise a github issue to support this.");

        self.seed ^= rapid_mix(self.seed ^ RAPID_SECRET[0], RAPID_SECRET[1]);
        let chunk = bytes;
        self.seed ^= chunk.len() as u64;
        let (a, b, seed) = rapidhash_core(self.a, self.b, self.seed, chunk);
        self.a = a;
        self.b = b;
        self.seed = seed;
        self.size += chunk.len() as u64;
    }
}
