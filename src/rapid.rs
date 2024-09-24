pub(crate) const RAPID_SEED: u64 = 0xbdd89aa982704029;
pub(crate) const RAPID_SECRET: [u64; 3] = [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

#[inline(always)]
pub(crate) fn rapidhash_raw(data: &[u8], mut seed: u64) -> u64 {
    seed ^= rapid_mix(seed ^ RAPID_SECRET[0], RAPID_SECRET[1]) ^ (data.len() as u64);
    let (a, b, _) = rapidhash_core(0, 0, seed, data);
    rapidhash_finish(a, b, data.len() as u64)
}

#[inline(always)]
pub(crate) fn rapid_mum(a: &mut u64, b: &mut u64) {
    let r = *a as u128 * *b as u128;
    *a = r as u64;
    *b = (r >> 64) as u64;
}

#[inline(always)]
pub(crate) fn rapid_mix(mut a: u64, mut b: u64) -> u64 {
    rapid_mum(&mut a, &mut b);
    a ^ b
}

#[inline(always)]
pub(crate) fn rapidhash_core(mut a: u64, mut b: u64, mut seed: u64, data: &[u8]) -> (u64, u64, u64) {
    if data.len() <= 16 {
        if data.len() >= 4 {
            let plast = data.len() - 4;
            a ^= ((read_u32(&data[0..]) as u64) << 32) | read_u32(&data[plast..]) as u64;
            let delta = (data.len() & 24) >> (data.len() >> 3);
            b ^= ((read_u32(&data[delta..]) as u64) << 32) | read_u32(&data[plast - delta..]) as u64;
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
            seed = rapid_mix(read_u64(&slice[0..]) ^ RAPID_SECRET[0], read_u64(&slice[8..]) ^ seed);
            see1 = rapid_mix(read_u64(&slice[16..]) ^ RAPID_SECRET[1], read_u64(&slice[24..]) ^ see1);
            see2 = rapid_mix(read_u64(&slice[32..]) ^ RAPID_SECRET[2], read_u64(&slice[40..]) ^ see2);
            seed = rapid_mix(read_u64(&slice[48..]) ^ RAPID_SECRET[0], read_u64(&slice[56..]) ^ seed);
            see1 = rapid_mix(read_u64(&slice[64..]) ^ RAPID_SECRET[1], read_u64(&slice[72..]) ^ see1);
            see2 = rapid_mix(read_u64(&slice[80..]) ^ RAPID_SECRET[2], read_u64(&slice[88..]) ^ see2);
            slice = &slice[96..];
        }
        if slice.len() >= 48 {
            seed = rapid_mix(read_u64(&slice[0..]) ^ RAPID_SECRET[0], read_u64(&slice[8..]) ^ seed);
            see1 = rapid_mix(read_u64(&slice[16..]) ^ RAPID_SECRET[1], read_u64(&slice[24..]) ^ see1);
            see2 = rapid_mix(read_u64(&slice[32..]) ^ RAPID_SECRET[2], read_u64(&slice[40..]) ^ see2);
            slice = &slice[48..];
        }
        seed ^= see1 ^ see2;

        if slice.len() > 16 {
            seed = rapid_mix(read_u64(&slice[0..]) ^ RAPID_SECRET[2], read_u64(&slice[8..]) ^ seed ^ RAPID_SECRET[1]);
            if slice.len() > 32 {
                seed = rapid_mix(read_u64(&slice[16..]) ^ RAPID_SECRET[2], read_u64(&slice[24..]) ^ seed);
            }
        }

        a ^= read_u64(&data[data.len()-16..]);
        b ^= read_u64(&data[data.len()-8..]);
    }

    a ^= RAPID_SECRET[1];
    b ^= seed;

    rapid_mum(&mut a, &mut b);
    (a, b, seed)
}

#[inline(always)]
pub(crate) fn rapidhash_finish(a: u64, b: u64, len: u64) -> u64 {
    rapid_mix(a ^ RAPID_SECRET[0] ^ len, b ^ RAPID_SECRET[1])
}

#[inline(always)]
#[allow(dead_code)]
pub(crate) fn read_u64(slice: &[u8]) -> u64 {
    let mut buf: [u8; 8] = Default::default();
    buf.copy_from_slice(&slice[..8]);
    u64::from_le_bytes(buf)
}

#[inline(always)]
pub(crate) fn read_u32(slice: &[u8]) -> u32 {
    let mut buf: [u8; 4] = Default::default();
    buf.copy_from_slice(&slice[..4]);
    u32::from_le_bytes(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_u32() {
        let bytes = &[23, 145, 3, 34];
        assert_eq!(read_u32(bytes), 570659095);

        assert_eq!(read_u32(&[0, 0, 0, 0]), 0);
        assert_eq!(read_u32(&[1, 0, 0, 0]), 1);
        assert_eq!(read_u32(&[12, 0, 0, 0]), 12);
        assert_eq!(read_u32(&[0, 10, 0, 0]), 2560);
    }

    #[test]
    fn test_bytes_u64() {
        let bytes = [23, 145, 3, 34, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes), 570659095);

        let bytes = [0, 0, 0, 0, 0, 0, 0, 0].as_slice();
        assert_eq!(read_u64(bytes), 0);
    }

    #[test]
    #[should_panic]
    fn test_bytes_u64_to_short_panics() {
        let bytes = [23, 145, 0].as_slice();
        assert_eq!(read_u64(bytes), 0);
    }

    #[test]
    fn test_rapid_mum() {
        let mut a = 0;
        let mut b = 0;
        rapid_mum(&mut a, &mut b);
        assert_eq!(a, 0);
        assert_eq!(b, 0);

        let mut a = 100;
        let mut b = 100;
        rapid_mum(&mut a, &mut b);
        assert_eq!(a, 10000);
        assert_eq!(b, 0);

        let mut a = u64::MAX;
        let mut b = 2;
        rapid_mum(&mut a, &mut b);
        assert_eq!(a, u64::MAX - 1);
        assert_eq!(b, 1);
    }
}
