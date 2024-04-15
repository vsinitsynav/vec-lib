use std::arch::x86_64::*;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec128b {
    xmm: __m128i,
}

impl Vec128b {
    pub const LEN: usize = 128;

    pub fn new() -> Self {
        Vec128b {
            xmm: unsafe { _mm_setzero_si128() },
        }
    }

    ///# Safety
    ///
    /// Loads from unaligned array
    #[inline]
    pub unsafe fn load(&mut self, mem_addr: *const u8) {
        self.xmm = _mm_loadu_si128(mem_addr as *const __m128i);
    }

    ///# Safety
    ///
    /// mem_addr must be aligned by 16
    #[inline]
    pub unsafe fn load_aligned(&mut self, mem_addr: *const u8) {
        self.xmm = _mm_load_si128(mem_addr as *const __m128i);
    }

    ///# Safety
    ///
    /// Stores into unaligned array
    #[inline]
    pub unsafe fn store(&self, mem_addr: *mut u8) {
        _mm_storeu_si128(mem_addr as *mut __m128i, self.xmm);
    }

    ///# Safety
    ///
    /// mem_addr must be aligned by 16
    #[inline]
    pub unsafe fn store_aligned(&self, mem_addr: *mut u8) {
        _mm_store_si128(mem_addr as *mut __m128i, self.xmm);
    }
}

impl Default for Vec128b {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::BitAnd for Vec128b {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Vec128b) -> Vec128b {
        Vec128b {
            xmm: unsafe { _mm_and_si128(self.xmm, other.xmm) },
        }
    }
}

impl ops::BitOr for Vec128b {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Vec128b) -> Vec128b {
        Vec128b {
            xmm: unsafe { _mm_or_si128(self.xmm, other.xmm) },
        }
    }
}

impl ops::BitXor for Vec128b {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: Vec128b) -> Vec128b {
        Vec128b {
            xmm: unsafe { _mm_xor_si128(self.xmm, other.xmm) },
        }
    }
}

impl ops::Not for Vec128b {
    type Output = Self;

    #[inline]
    fn not(self) -> Vec128b {
        Vec128b {
            xmm: unsafe { _mm_xor_si128(self.xmm, _mm_set1_epi32(-1)) },
        }
    }
}

impl ops::BitAndAssign for Vec128b {
    #[inline]
    fn bitand_assign(&mut self, other: Vec128b) {
        self.xmm = unsafe { _mm_and_si128(self.xmm, other.xmm) };
    }
}

impl ops::BitOrAssign for Vec128b {
    #[inline]
    fn bitor_assign(&mut self, other: Vec128b) {
        self.xmm = unsafe { _mm_or_si128(self.xmm, other.xmm) };
    }
}

impl ops::BitXorAssign for Vec128b {
    #[inline]
    fn bitxor_assign(&mut self, other: Vec128b) {
        self.xmm = unsafe { _mm_xor_si128(self.xmm, other.xmm) };
    }
}

///# Safety
///
/// function andnot: a & ~ b
#[inline]
pub fn andnot(a: Vec128b, b: Vec128b) -> Vec128b {
    unsafe {
        Vec128b {
            xmm: _mm_andnot_si128(b.xmm, a.xmm),
        }
    }
}

///# Safety
///
/// Select between two sources, byte by byte, using broad boolean vector s.
/// Corresponds to this pseudocode:
/// for (int i = 0; i < 16; i++) result[i] = s[i] ? a[i] : b[i];
/// Each byte in s must be either 0 (false) or 0xFF (true). No other values are allowed.
#[inline]
pub(crate) fn selectb(s: __m128i, a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_and_si128(s, a), _mm_andnot_si128(s, b)) }
}

///# Safety
///
/// Returns false if at least one bit is 0
#[inline]
pub fn horizontal_and(a: Vec128b) -> bool {
    unsafe {
        let t1 = _mm_unpackhi_epi64(a.xmm, a.xmm);
        let t2 = _mm_and_si128(a.xmm, t1);
        _mm_cvtsi128_si64(t2) == -1
    }
}

///# Safety
///
/// Returns true if at least one bit is 1
#[inline]
pub fn horizontal_or(a: Vec128b) -> bool {
    unsafe {
        let t1 = _mm_unpackhi_epi64(a.xmm, a.xmm);
        let t2 = _mm_or_si128(a.xmm, t1);
        _mm_cvtsi128_si64(t2) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec128b() {
        unsafe {
            let mut arr: [u8; 16] = [0; 16];
            let mut a128 = Vec128b::new();
            a128.store(&mut arr as *mut u8);
            assert_eq!(false, horizontal_or(a128));

            arr[0] = 1;
            a128.load(&arr as *const u8);
            assert_eq!(true, horizontal_or(a128));
            assert_eq!(false, horizontal_and(a128));

            a128 ^= a128;
            assert_eq!(false, horizontal_or(a128));

            // writeln!({a128});
        }
    }
}
