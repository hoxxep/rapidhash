#[cfg(feature = "rng")]
use rand_core::{RngCore, SeedableRng, Error, impls};
use crate::rapid_const::{rapid_mix, rapidhash_core, rapidhash_finish, rapidhash_seed, RAPID_SECRET};

/// Generate the next random number in a sequence using the rapidhash mixing algorithm.
///
/// This is a weaker RNG than the full RapidRng implementation, as it is a single chain through the
/// u64 space, but is 1000x faster. It's equivalent in logic and performance to
/// [wyhash::wyrng](https://docs.rs/wyhash/latest/wyhash/fn.wyrng.html) and
/// [fasthash::u64](https://docs.rs/fastrand/latest/fastrand/) but uses rapidhash
/// constants/secrets.
///
/// The weakness with this RNG is that at best it's a single cycle over the u64 space, as the seed
/// is simple a position in a constant sequence. Use the [rapidrng_quality] method for a higher
/// entropy RNG with multiple dimensions.
#[inline]
pub fn rapidrng_fast(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(RAPID_SECRET[0]);
    rapid_mix(*seed, *seed ^ RAPID_SECRET[1])
}

/// A higher quality random number generator that uses the full rapidhash algorithm.
///
/// The state is a `[u64; 3]` for generating the next random number, allowing different seeds to
/// produce different _sequences_.
///
/// This can be up to 10x slower than `rapidrng_fast` for a higher entropy non-cryptographic RNG.
#[inline]
pub fn rapidrng_quality(state: &mut [u64; 3]) -> u64 {
    state[0] = state[0].wrapping_add(RAPID_SECRET[0]);
    state[1] ^= RAPID_SECRET[1];
    state[2] ^= state[0];
    let (a, b) = crate::rapid_const::rapid_mum(state[1], state[2]);
    state[1] = a;
    state[2] = b;
    state[1] ^ state[2]
}

/// Generate a random number non-deterministically by re-seeding with the current time.
///
/// Note fetching system time requires a syscall and is therefore much slower than [rapidrng_fast].
/// It can also be used to seed [rapidrng_fast].
///
/// Requires the `std` feature and a platform that supports [std::time::SystemTime].
///
/// # Example
/// ```rust
/// use rapidhash::{rapidrng_fast, rapidrng_time};
///
/// // choose a non-deterministic random seed (50-100ns)
/// let mut seed = rapidrng_time(&mut 0);
///
/// // rapid fast deterministic random numbers (~1ns/iter)
/// for _ in 0..10 {
///     println!("{}", rapidrng_fast(&mut seed));
/// }
/// ```
#[cfg(feature = "std")]
#[inline]
pub fn rapidrng_time(seed: &mut u64) -> u64 {
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    // NOTE limited entropy: only a few of the time.as_secs bits will change between calls, and the
    // time.subsec_nanos may only have milli- or micro-second precision on some platforms.
    // This is why we further stretch the teed with multiple rounds of rapid_mix.
    let mut  teed = ((time.as_secs() as u64) << 32) | time.subsec_nanos() as u64;
    teed = rapid_mix(teed ^ RAPID_SECRET[0], *seed ^ RAPID_SECRET[1]);
    *seed = rapid_mix(teed ^ RAPID_SECRET[0], RAPID_SECRET[2]);
    rapid_mix(*seed, *seed ^ RAPID_SECRET[1])
}

/// A random number generator that uses the rapidhash mixing algorithm.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct RapidRng {
    seed: u64,
    a: u64,
    b: u64,
}

impl RapidRng {
    /// Create a new random number generator from a seed.
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            a: 0,
            b: 0,
        }
    }

    /// Export the current state of the random number generator.
    #[inline]
    pub fn state(&self) -> [u8; 24] {
        let mut state = [0; 24];
        state[0..8].copy_from_slice(&self.seed.to_le_bytes());
        state[8..16].copy_from_slice(&self.a.to_le_bytes());
        state[16..24].copy_from_slice(&self.b.to_le_bytes());
        state
    }

    #[inline]
    pub fn next(&mut self) -> u64 {
        // mix a new seed
        let seed = rapidhash_seed(self.seed, self.a);

        // mix a and b, using the old and new seed
        let (a, b, _) = rapidhash_core(self.a, self.b, seed, &self.seed.to_le_bytes());
        self.a = a;
        self.b = b;
        self.seed = seed;

        // mix a, b, and seed to get the next random number
        rapidhash_finish(a, b, seed)
    }
}

#[cfg(feature = "rng")]
impl RngCore for RapidRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.next()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(feature = "rng")]
impl SeedableRng for RapidRng {
    type Seed = [u8; 24];

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            seed: u64::from_le_bytes(seed[0..8].try_into().unwrap()),
            a: u64::from_le_bytes(seed[8..16].try_into().unwrap()),
            b: u64::from_le_bytes(seed[16..24].try_into().unwrap()),
        }
    }

    #[inline]
    fn seed_from_u64(state: u64) -> Self {
        Self::new(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "rng")]
    #[test]
    fn test_rapidrng() {
        let mut rng = RapidRng::new(0);
        let x = rng.next();
        let y = rng.next();
        assert_ne!(x, 0);
        assert_ne!(x, y);
    }

    #[cfg(all(feature = "rng", feature = "std"))]
    #[test]
    fn bit_flip_trial() {
        let cycles = 100_000;
        let mut seen = std::collections::HashSet::with_capacity(cycles);
        let mut flips = std::vec::Vec::with_capacity(cycles);
        let mut rng = RapidRng::new(0);

        let mut prev = 0;
        for _ in 0..cycles {
            let next = rng.next_u64();

            let xor = prev ^ next;
            let flipped = xor.count_ones() as u64;
            assert!(xor.count_ones() >= 12, "Flipping bit changed only {} bits", flipped);
            flips.push(flipped);

            assert!(!seen.contains(&next), "RapidRngFast produced a duplicate value");
            seen.insert(next);

            prev = next;
        }

        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {}, expected: 32.0", average);
    }

    #[cfg(feature = "std")]
    #[test]
    fn bit_flip_trial_fast() {
        let cycles = 100_000;
        let mut seen = std::collections::HashSet::with_capacity(cycles);
        let mut flips = std::vec::Vec::with_capacity(cycles);

        let mut prev = 0;
        for _ in 0..cycles {
            let next = rapidrng_fast(&mut prev);

            let xor = prev ^ next;
            let flipped = xor.count_ones() as u64;
            assert!(xor.count_ones() >= 12, "Flipping bit changed only {} bits", flipped);
            flips.push(flipped);

            assert!(!seen.contains(&next), "rapidrng_fast produced a duplicate value");
            seen.insert(next);

            prev = next;
        }

        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {}, expected: 32.0", average);
    }

    #[cfg(feature = "std")]
    #[test]
    fn bit_flip_trial_time() {
        let cycles = 100_000;
        let mut seen = std::collections::HashSet::with_capacity(cycles);
        let mut flips = std::vec::Vec::with_capacity(cycles);

        let mut prev = 0;
        for _ in 0..cycles {
            let next = rapidrng_time(&mut prev);

            let xor = prev ^ next;
            let flipped = xor.count_ones() as u64;
            assert!(xor.count_ones() >= 12, "Flipping bit changed only {} bits", flipped);
            flips.push(flipped);

            assert!(!seen.contains(&next), "rapidrng_fast produced a duplicate value");
            seen.insert(next);

            prev = next;
        }

        let average = flips.iter().sum::<u64>() as f64 / flips.len() as f64;
        assert!(average > 31.95 && average < 32.05, "Did not flip an average of half the bits. average: {}, expected: 32.0", average);
    }

    /// detects a cycle at: 4294967296:1751221902
    /// note that we're detecting _seed_ cycles, not output values.
    #[test]
    #[ignore]
    fn find_cycle() {
        let mut fast = 0;
        let mut slow = 0;

        let mut power: u64 = 1;
        let mut lam: u64 = 1;
        rapidrng_fast(&mut fast);
        while fast != slow {
            if power == lam {
                slow = fast;
                power *= 2;
                lam = 0;
            }
            rapidrng_fast(&mut fast);
            lam += 1;
        }

        assert!(false, "Cycle found after {power}:{lam} iterations.");
    }

    #[cfg(feature = "rng")]
    #[test]
    #[ignore]
    fn find_cycle_slow() {
        let mut rng = RapidRng::new(0);

        let mut power: u64 = 1;
        let mut lam: u64 = 1;
        let mut fast = rng.next_u64();
        let mut slow = 0;
        while fast != slow {
            if power == lam {
                slow = fast;
                power *= 2;
                lam = 0;
            }
            fast = rng.next_u64();
            lam += 1;
        }

        assert!(false, "Cycle found after {power}:{lam} iterations.");
    }

    /// detects a cycle at: 2147483648:1605182499
    /// note that we're detecting _seed_ cycles, not output values.
    #[test]
    #[ignore]
    fn find_cycle_wyhash() {
        let mut fast = 0;
        let mut slow = 0;

        let mut power: u64 = 1;
        let mut lam: u64 = 1;
        wyhash::wyrng(&mut fast);
        while fast != slow {
            if power == lam {
                slow = fast;
                power *= 2;
                lam = 0;
            }
            wyhash::wyrng(&mut fast);
            lam += 1;
        }

        assert!(false, "Cycle found after {power}:{lam} iterations.");
    }

    #[cfg(feature = "rng")]
    #[test]
    fn test_construction() {
        let mut rng = RapidRng::default();
        assert_ne!(rng.next(), 0);
    }
}
