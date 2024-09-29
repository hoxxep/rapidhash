/// The rapidhash default seed.
pub const RAPID_SEED: u64 = 0xbdd89aa982704029;
pub(crate) const RAPID_SECRET: [u64; 3] = [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

/// Rapidhash a single byte stream, matching the C++ implementation.
#[inline]
pub const fn rapidhash(data: &[u8]) -> u64 {
    rapidhash_inline(data, RAPID_SEED)
}

/// Rapidhash a single byte stream, matching the C++ implementation, with a custom seed.
#[inline]
pub const fn rapidhash_seeded(data: &[u8], seed: u64) -> u64 {
    rapidhash_inline(data, seed)
}

/// Rapidhash a single byte stream, matching the C++ implementation.
///
/// Is marked with `#[inline(always)]` to force the compiler to inline and optimise the method.
/// Can provide large performance uplifts for inputs where the length is known at compile time.
#[inline(always)]
pub const fn rapidhash_inline(data: &[u8], mut seed: u64) -> u64 {
    seed = rapidhash_seed(seed, data.len() as u64);
    let (a, b, _) = rapidhash_core(0, 0, seed, data);
    rapidhash_finish(a, b, data.len() as u64)
}

#[inline(always)]
pub const fn rapid_mum(a: u64, b: u64) -> (u64, u64) {
    let r = a as u128 * b as u128;
    (r as u64, (r >> 64) as u64)
}

#[inline(always)]
pub const fn rapid_mix(a: u64, b: u64) -> u64 {
    let (a, b) = rapid_mum(a, b);
    a ^ b
}

#[inline(always)]
pub(crate) const fn rapidhash_seed(seed: u64, len: u64) -> u64 {
    seed ^ rapid_mix(seed ^ RAPID_SECRET[0], RAPID_SECRET[1]) ^ len
}

#[inline(always)]
pub(crate) const fn rapidhash_core(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    if data.len() <= 16 {
        // deviation from the C++ impl computes delta as follows
        // let delta = (data.len() & 24) >> (data.len() >> 3);
        // this is equivalent to "match {..8=>0, 8..=>4}"
        // and so using the extra if-else statement is equivalent and allows the compiler to skip
        // some unnecessary bounds checks while still being safe rust.
        if data.len() >= 8 {
            // len is 4..=16
            let plast = data.len() - 4;
            let delta = 4;
            a ^= read_u32_combined(data, 0, plast);
            b ^= read_u32_combined(data, delta, plast - delta);
        } else if data.len() >= 4 {
            let plast = data.len() - 4;
            let delta = 0;
            a ^= read_u32_combined(data, 0, plast);
            b ^= read_u32_combined(data, delta, plast - delta);
        } else if data.len() > 0 {
            // len is 1..=3
            let len = data.len();
            a ^= ((data[0] as u64) << 56) | ((data[len >> 1] as u64) << 32) | data[len - 1] as u64;
            // b = 0;
        }
    } else {
        let mut slice = data;

        // most CPUs appear to benefit from this unrolled loop
        let mut see1 = seed;
        let mut see2 = seed;
        while slice.len() >= 96 {
            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            seed = rapid_mix(read_u64(slice , 48) ^ RAPID_SECRET[0], read_u64(slice, 56) ^ seed);
            see1 = rapid_mix(read_u64(slice, 64) ^ RAPID_SECRET[1], read_u64(slice, 72) ^ see1);
            see2 = rapid_mix(read_u64(slice, 80) ^ RAPID_SECRET[2], read_u64(slice, 88) ^ see2);
            let (_, split) = slice.split_at(96);
            slice = split;
        }
        if slice.len() >= 48 {
            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[0], read_u64(slice, 8) ^ seed);
            see1 = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[1], read_u64(slice, 24) ^ see1);
            see2 = rapid_mix(read_u64(slice, 32) ^ RAPID_SECRET[2], read_u64(slice, 40) ^ see2);
            let (_, split) = slice.split_at(48);
            slice = split;
        }
        seed ^= see1 ^ see2;

        if slice.len() > 16 {
            seed = rapid_mix(read_u64(slice, 0) ^ RAPID_SECRET[2], read_u64(slice, 8) ^ seed ^ RAPID_SECRET[1]);
            if slice.len() > 32 {
                seed = rapid_mix(read_u64(slice, 16) ^ RAPID_SECRET[2], read_u64(slice, 24) ^ seed);
            }
        }

        a ^= read_u64(data, data.len() - 16);
        b ^= read_u64(data, data.len() - 8);
    }

    a ^= RAPID_SECRET[1];
    b ^= seed;

    let (a2, b2) = rapid_mum(a, b);
    a = a2;
    b = b2;
    (a, b, seed)
}

#[inline(always)]
pub(crate) const fn rapidhash_finish(a: u64, b: u64, len: u64) -> u64 {
    rapid_mix(a ^ RAPID_SECRET[0] ^ len, b ^ RAPID_SECRET[1])
}

/// Hacky const-friendly memory-safe unaligned bytes to u64. Compiler can't seem to remove the
/// bounds check, and so we have an unsafe version behind the `unsafe` feature flag.
#[cfg(not(feature = "unsafe"))]
#[inline(always)]
const fn read_u64(slice: &[u8], offset: usize) -> u64 {
    // equivalent to slice[offset..offset+8].try_into().unwrap(), but const-friendly
    let maybe_buf = slice.split_at(offset).1.first_chunk::<8>();
    let buf = match maybe_buf {
        Some(buf) => *buf,
        None => panic!("read_u64: slice too short"),
    };
    u64::from_le_bytes(buf)
}

/// Hacky const-friendly memory-safe unaligned bytes to u64. Compiler can't seem to remove the
/// bounds check, and so we have an unsafe version behind the `unsafe` feature flag.
#[cfg(not(feature = "unsafe"))]
#[inline(always)]
const fn read_u32(slice: &[u8], offset: usize) -> u32 {
    // equivalent to slice[offset..offset+4].try_into().unwrap(), but const-friendly
    let maybe_buf = slice.split_at(offset).1.first_chunk::<4>();
    let buf = match maybe_buf {
        Some(buf) => *buf,
        None => panic!("read_u32: slice too short"),
    };
    u32::from_le_bytes(buf)
}

/// Unsafe but const-friendly unaligned bytes to u64. The compiler can't seem to remove the bounds
/// checks for small integers because we do some funky bit shifting in the indexing.
///
/// SAFETY: `slice` must be at least `offset+8` bytes long, which we guarantee in this rapidhash
/// implementation.
#[cfg(feature = "unsafe")]
#[inline(always)]
const fn read_u64(slice: &[u8], offset: usize) -> u64 {
    debug_assert!(offset as isize >= 0);
    debug_assert!(slice.len() >= 8 + offset);
    let val = unsafe { std::ptr::read_unaligned(slice.as_ptr().offset(offset as isize) as *const u64) };
    val.to_le()  // swap bytes on big-endian systems to get the same u64 value
}

/// Unsafe but const-friendly unaligned bytes to u32. The compiler can't seem to remove the bounds
/// checks for small integers because we do some funky bit shifting in the indexing.
///
/// SAFETY: `slice` must be at least `offset+8` bytes long, which we guarantee in this rapidhash
/// implementation.
#[cfg(feature = "unsafe")]
#[inline(always)]
const fn read_u32(slice: &[u8], offset: usize) -> u32 {
    debug_assert!(offset as isize >= 0);
    debug_assert!(slice.len() >= 4 + offset);
    let val = unsafe { std::ptr::read_unaligned(slice.as_ptr().offset(offset as isize) as *const u32) };
    val.to_le()  // swap bytes on big-endian systems to get the same u64 value
}

#[inline(always)]
const fn read_u32_combined(slice: &[u8], offset_top: usize, offset_bot: usize) -> u64 {
    debug_assert!(slice.len() >= 4 + offset_top && slice.len() >= 4 + offset_bot);
    let top = read_u32(slice, offset_top) as u64;
    let bot = read_u32(slice, offset_bot) as u64;
    (top << 32) | bot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u32() {
        let bytes = &[23, 145, 3, 34];

        let split_result = bytes.split_at(0).1;
        assert_eq!(split_result.len(), 4);
        let maybe_buf = split_result.first_chunk::<4>();
        assert_eq!(maybe_buf, Some(&[23, 145, 3, 34]));

        assert_eq!(read_u32(bytes, 0), 570659095);

        let bytes = &[24, 54, 3, 23, 145, 3, 34];
        assert_eq!(read_u32(bytes, 3), 570659095);

        assert_eq!(read_u32(&[0, 0, 0, 0], 0), 0);
        assert_eq!(read_u32(&[1, 0, 0, 0], 0), 1);
        assert_eq!(read_u32(&[12, 0, 0, 0], 0), 12);
        assert_eq!(read_u32(&[0, 10, 0, 0], 0), 2560);
    }

    #[test]
    fn test_read_u64() {
        let bytes = [23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes, 0), 570659095);

        let bytes = [1, 2, 3, 23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes, 3), 570659095);

        let bytes = [0, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes, 0), 0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_u32_to_u128_delta() {
        fn formula(len: u64) -> u64 {
            (len & 24) >> (len >> 3)
        }

        fn formula2(len: u64) -> u64 {
            match len {
                8.. => 4,
                _ => 0,
            }
        }

        let inputs: std::vec::Vec<u64> = (4..=16).collect();
        let outputs: std::vec::Vec<u64> = inputs.iter().map(|&x| formula(x)).collect();
        let expected = std::vec![0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4];
        assert_eq!(outputs, expected);
        assert_eq!(outputs, inputs.iter().map(|&x| formula2(x)).collect::<Vec<u64>>());
    }

    #[test]
    #[should_panic]
    #[cfg(any(test, not(feature = "unsafe")))]
    fn test_read_u32_to_short_panics() {
        let bytes = [23, 145, 0].as_slice();
        assert_eq!(read_u32(bytes, 0), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(any(test, not(feature = "unsafe")))]
    fn test_read_u64_to_short_panics() {
        let bytes = [23, 145, 0].as_slice();
        assert_eq!(read_u64(bytes, 0), 0);
    }

    #[test]
    fn test_rapid_mum() {
        let (a, b) = rapid_mum(0, 0);
        assert_eq!(a, 0);
        assert_eq!(b, 0);

        let (a, b) = rapid_mum(100, 100);
        assert_eq!(a, 10000);
        assert_eq!(b, 0);

        let (a, b) = rapid_mum(u64::MAX, 2);
        assert_eq!(a, u64::MAX - 1);
        assert_eq!(b, 1);
    }
}
