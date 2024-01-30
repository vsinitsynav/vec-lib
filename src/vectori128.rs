use std::arch::x86_64::*;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec128b {
    xmm: __m128i,
}

impl Vec128b {
    pub fn new() -> Self {
        Vec128b {
            xmm: unsafe { _mm_setzero_si128() },
        }
    }

    pub const fn build(x: __m128i) -> Self {
        Vec128b { xmm: x }
    }

    ///# Safety
    ///
    /// TODO
    pub unsafe fn load(&mut self, mem_addr: *const __m128i) -> &Self {
        self.xmm = _mm_loadu_si128(mem_addr);
        self
    }

    ///# Safety
    ///
    /// TODO
    pub unsafe fn load_a(&mut self, mem_addr: *const __m128i) {
        self.xmm = _mm_load_si128(mem_addr);
    }

    ///# Safety
    ///
    /// TODO
    pub unsafe fn store(&self, mem_addr: *mut __m128i) {
        _mm_storeu_si128(mem_addr, self.xmm);
    }

    ///# Safety
    ///
    /// TODO
    pub unsafe fn store_a(&self, mem_addr: *mut __m128i) {
        _mm_store_si128(mem_addr, self.xmm);
    }

    ///# Safety
    ///
    /// TODO
    pub unsafe fn store_nt(&self, mem_addr: *mut __m128i) {
        _mm_stream_si128(mem_addr, self.xmm);
    }

    pub fn size(&self) -> usize {
        128
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
/// TODO
#[inline]
pub unsafe fn andnot(a: Vec128b, b: Vec128b) -> Vec128b {
    Vec128b {
        xmm: _mm_andnot_si128(b.xmm, a.xmm),
    }
}

///# Safety
///
/// TODO
#[inline]
pub unsafe fn selectb(s: __m128i, a: __m128i, b: __m128i) -> __m128i {
    _mm_or_si128(_mm_and_si128(s, a), _mm_andnot_si128(s, b))
}

///# Safety
///
/// TODO
#[inline]
pub unsafe fn horizontal_and(a: Vec128b) -> bool {
    let t1 = _mm_unpackhi_epi64(a.xmm, a.xmm);
    let t2 = _mm_and_si128(a.xmm, t1);
    _mm_cvtsi128_si64(t2) == -1
}

///# Safety
///
/// TODO
#[inline]
pub unsafe fn horizontal_or(a: Vec128b) -> bool {
    let t1 = _mm_unpackhi_epi64(a.xmm, a.xmm);
    let t2 = _mm_or_si128(a.xmm, t1);
    _mm_cvtsi128_si64(t2) != 0
}
