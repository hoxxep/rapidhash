#[allow(unused)]
pub(crate) const RAPID_SEED: u64 = 0xbdd89aa982704029;
#[allow(unused)]
pub(crate) const RAPID_SECRET: [u64; 3] = [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

#[inline(always)]
pub const fn rapidhash_raw(data: &[u8], mut seed: u64) -> u64 {
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
        if data.len() >= 4 {
            let plast = data.len() - 4;
            a ^= ((read_u32(data, 0) as u64) << 32) | read_u32(data, plast) as u64;
            let delta = (data.len() & 24) >> (data.len() >> 3);
            b ^= ((read_u32(data, delta) as u64) << 32) | read_u32(data, plast - delta) as u64;
        } else if data.len() > 0 {
            // len is bounded between 1 and 3
            let len = data.len();
            a ^= ((data[0] as u64) << 56) | ((data[len >> 1] as u64) << 32) | data[len - 1] as u64;
            // b = 0;
        }
    } else {
        let mut slice = data;

        // "most CPUs benefit from this unrolled loop"
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
    let buf: [u8; 8] = [slice[0 + offset], slice[1 + offset], slice[2 + offset], slice[3 + offset], slice[4 + offset], slice[5 + offset], slice[6 + offset], slice[7 + offset]];
    u64::from_le_bytes(buf)
}

/// Hacky const-friendly memory-safe unaligned bytes to u64. Compiler can't seem to remove the
/// bounds check, and so we have an unsafe version behind the `unsafe` feature flag.
#[cfg(not(feature = "unsafe"))]
#[inline(always)]
const fn read_u32(slice: &[u8], offset: usize) -> u32 {
    let buf: [u8; 4] = [slice[0 + offset], slice[1 + offset], slice[2 + offset], slice[3 + offset]];
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
    debug_assert!(slice.len() >= 4 + offset);
    unsafe { std::ptr::read_unaligned(slice.as_ptr().offset(offset as isize) as *const u64) }
}


/// Unsafe but const-friendly unaligned bytes to u32. The compiler can't seem to remove the bounds
/// checks for small integers because we do some funky bit shifting in the indexing.
///
/// SAFETY: `slice` must be at least `offset+8` bytes long, which we guarantee in this rapidhash
/// implementation.
#[cfg(feature=  "unsafe")]
#[inline(always)]
const fn read_u32(slice: &[u8], offset: usize) -> u32 {
    debug_assert!(offset as isize >= 0);
    debug_assert!(slice.len() >= 4 + offset);
    unsafe { std::ptr::read_unaligned(slice.as_ptr().offset(offset as isize) as *const u32) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_u32() {
        let bytes = &[23, 145, 3, 34];
        assert_eq!(read_u32(bytes, 0), 570659095);

        let bytes = &[24, 54, 3, 23, 145, 3, 34];
        assert_eq!(read_u32(bytes, 3), 570659095);

        assert_eq!(read_u32(&[0, 0, 0, 0], 0), 0);
        assert_eq!(read_u32(&[1, 0, 0, 0], 0), 1);
        assert_eq!(read_u32(&[12, 0, 0, 0], 0), 12);
        assert_eq!(read_u32(&[0, 10, 0, 0], 0), 2560);
    }

    #[test]
    fn test_bytes_u64() {
        let bytes = [23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes, 0), 570659095);

        let bytes = [1, 2, 3, 23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes, 3), 570659095);

        let bytes = [0, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes, 0), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(any(test, not(feature = "unsafe")))]
    fn test_bytes_u64_to_short_panics() {
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
