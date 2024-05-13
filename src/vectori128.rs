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

    ///# Safety: obvious
    ///
    /// Loads from unaligned array
    #[inline]
    pub unsafe fn load(&mut self, mem_addr: *const u8) {
        self.xmm = _mm_loadu_si128(mem_addr as *const __m128i);
    }

    ///# Safety: obvious
    ///
    /// mem_addr must be aligned by 16
    #[inline]
    pub unsafe fn load_aligned(&mut self, mem_addr: *const u8) {
        self.xmm = _mm_load_si128(mem_addr as *const __m128i);
    }

    ///# Safety: obvious
    ///
    /// Stores into unaligned array
    #[inline]
    pub unsafe fn store(&self, mem_addr: *mut u8) {
        _mm_storeu_si128(mem_addr as *mut __m128i, self.xmm);
    }

    ///# Safety: obvious
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

///# Safety: obvious
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

///# Safety: each byte in s must be either 0 (false) or 0xFF (true).
/// No other values are allowed.
///
/// Select between two sources, byte by byte, using broad boolean vector s.
/// Corresponds to this pseudocode:
/// for (int i = 0; i < 16; i++) result[i] = s[i] ? a[i] : b[i];
#[inline]
pub(crate) unsafe fn selectb(s: __m128i, a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_or_si128(_mm_and_si128(s, a), _mm_andnot_si128(s, b)) }
}

///# Safety: obvious
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

///# Safety: obvious
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

#[derive(Copy, Clone, Debug)]
pub struct Vec16c {
    xmm: __m128i,
}

impl Vec16c {
    pub const LEN: usize = 16;

    pub fn new() -> Self {
        Vec16c {
            xmm: unsafe { _mm_setzero_si128() },
        }
    }

    /// Constructor to broadcast the same value into all elements:
    pub fn set_value(a: i8) -> Self {
        Vec16c {
            xmm: unsafe { _mm_set1_epi8(a) },
        }
    }

    /// Constructor to build from all elements:
    pub fn set_values(a: [i8; 16]) -> Self {
        unsafe {
            Vec16c {
                xmm: _mm_loadu_si128(&a as *const i8 as *const __m128i),
            }
        }
    }

    ///# Safety: obvious
    ///
    /// Member function to load from array (unaligned)
    #[inline]
    pub unsafe fn load(&mut self, mem_addr: *const i8) {
        self.xmm = _mm_loadu_si128(mem_addr as *const __m128i);
    }

    ///# Safety: obvious
    ///
    /// mem_addr must be aligned by 16
    #[inline]
    pub unsafe fn load_aligned(&mut self, mem_addr: *const i8) {
        self.xmm = _mm_load_si128(mem_addr as *const __m128i);
    }

    ///# Safety: obvious
    ///
    /// Stores into unaligned array
    #[inline]
    pub unsafe fn store(&self, mem_addr: *mut i8) {
        _mm_storeu_si128(mem_addr as *mut __m128i, self.xmm);
    }

    ///# Safety: obvious
    ///
    /// mem_addr must be aligned by 16
    #[inline]
    pub unsafe fn store_aligned(&self, mem_addr: *mut i8) {
        _mm_store_si128(mem_addr as *mut __m128i, self.xmm);
    }

    ///# Safety: n bytes in the mem_addr must be valid
    ///
    /// Partial load. Load n elements and set the rest to 0
    #[inline]
    pub unsafe fn load_partial(&mut self, n: usize, arr: &[i8]) {
        if n >= 16 {
            self.load(arr as *const [i8] as *const i8);
        } else if n <= 0 {
            self.xmm = _mm_setzero_si128();
        } else if ((arr as *const [i8] as *const i8) as i32 & 0xFFF) < 0xFF0 {
            // mem_addr is at least 16 bytes from a page boundary. OK to read 16 bytes
            self.load(arr as *const [i8] as *const i8);
        } else {
            // worst case. read 1 byte at a time and suffer store forwarding penalty
            // unless the compiler can optimize this
            let mut x: [i8; 16] = [0; 16];
            for i in 0..n {
                x[i] = arr[i];
            }
            self.load(&x as *const i8);
        }
        self.cutoff(n);
    }

    ///# Safety: at least n bytes must be allocated to the mem_addr
    ///
    /// Partial store. Store n elements
    #[inline]
    pub unsafe fn store_partial(&mut self, mut n: usize, mem_addr: *mut i8) {
        let mut s: [i8; 16] = [0; 16];
        self.store(&mut s as *mut i8);
        if n as u32 > 16 {
            n = 16;
        }
        for i in 0..n {
            *(mem_addr).offset(i.try_into().unwrap()) = s[i];
        }
    }

    ///# Safety: obvious
    ///
    /// cut off vector to n elements. The last 16-n elements are set to zero
    #[inline]
    pub unsafe fn cutoff(&mut self, n: usize) {
        if n as u32 >= 16 {
            return;
        }

        let mask: [i8; 32] = core::array::from_fn(|i| ((i as i8 & 16) >> 4) - 1);
        let mut tmp = Vec16c::new();
        tmp.load((&mask as *const i8).offset(16 - n as isize));
        self.xmm = _mm_and_si128(self.xmm, tmp.xmm)
    }

    ///# Safety: obvious
    ///
    /// Member function to change a single element in vector
    #[inline]
    pub unsafe fn insert(&mut self, index: isize, value: i8) {
        let mut maskl: [i8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let broad = _mm_set1_epi8(value);
        let mask =
            _mm_loadu_si128((&mut maskl as *mut i8).offset(16 - (index & 0x0F)) as *const __m128i); // mask with FF at index position
        self.xmm = selectb(mask, broad, self.xmm);
    }

    ///# Safety: obvious
    ///
    /// Member function extract a single element from vector
    #[inline]
    pub unsafe fn extract(&mut self, index: usize) -> i8 {
        let mut x: [i8; 16] = [0; 16];
        self.store(&mut x as *mut i8);
        x[index & 0x0F]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec16bc {
    xmm: __m128i,
}

impl Vec16bc {
    pub fn set_values(
        x0: bool,
        x1: bool,
        x2: bool,
        x3: bool,
        x4: bool,
        x5: bool,
        x6: bool,
        x7: bool,
        x8: bool,
        x9: bool,
        x10: bool,
        x11: bool,
        x12: bool,
        x13: bool,
        x14: bool,
        x15: bool,
    ) -> Self {
        Vec16bc {
            xmm: Vec16c::set_values([
                -(x0 as i8),
                -(x1 as i8),
                -(x2 as i8),
                -(x3 as i8),
                -(x4 as i8),
                -(x5 as i8),
                -(x6 as i8),
                -(x7 as i8),
                -(x8 as i8),
                -(x9 as i8),
                -(x10 as i8),
                -(x11 as i8),
                -(x12 as i8),
                -(x13 as i8),
                -(x14 as i8),
                -(x15 as i8),
            ])
            .xmm,
        }
    }

    pub fn set_value(x: __m128i) -> Self {
        Vec16bc { xmm: x }
    }
}

/*****************************************************************************
*
*          Define operators for Vec16c
*
*****************************************************************************/

/// Convert Vec16bc to Vec16c
impl From<Vec16bc> for Vec16c {
    fn from(a: Vec16bc) -> Self {
        Vec16c { xmm: a.xmm }
    }
}

/// vector operator + : add element by element
impl ops::Add for Vec16c {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            xmm: unsafe { _mm_add_epi8(self.xmm, other.xmm) },
        }
    }
}

/// vector operator += : add
impl ops::AddAssign for Vec16c {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

/// vector operator - : subtract element by element
impl ops::Sub for Vec16c {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            xmm: unsafe { _mm_sub_epi8(self.xmm, other.xmm) },
        }
    }
}

/// vector operator - : unary minus
impl ops::Neg for Vec16c {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            xmm: unsafe { _mm_sub_epi8(_mm_setzero_si128(), self.xmm) },
        }
    }
}

///vector operator -= : add
impl ops::SubAssign for Vec16c {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

/// vector operator * : multiply element by element
impl ops::Mul for Vec16c {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let aodd = unsafe { _mm_srli_epi16(self.xmm, 8) };
        let bodd = unsafe { _mm_srli_epi16(other.xmm, 8) };
        let muleven = unsafe { _mm_mullo_epi16(self.xmm, other.xmm) };
        let mulodd = unsafe { _mm_mullo_epi16(aodd, bodd) };
        let mulodd = unsafe { _mm_slli_epi16(mulodd, 8) };
        let mask = unsafe { _mm_set1_epi32(0x00FF00FF) };
        Self {
            xmm: unsafe { selectb(mask, muleven, mulodd) },
        }
    }
}

/// vector operator *= : multiply
impl ops::MulAssign for Vec16c {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

/// vector operator << : shift left all elements
impl ops::Shl<i32> for Vec16c {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        unsafe {
            let mask = (0xFF as u32) >> (rhs as u32);
            let am = _mm_and_si128(self.xmm, _mm_set1_epi8(mask as i8));
            Self {
                xmm: _mm_sll_epi16(am, _mm_cvtsi32_si128(rhs)),
            }
        }
    }
}

/// vector operator <<= : shift left
impl ops::ShlAssign<i32> for Vec16c {
    #[inline]
    fn shl_assign(&mut self, rhs: i32) {
        *self = *self << rhs
    }
}

/// vector operator >> : shift right arithmetic all elements
impl ops::Shr<i32> for Vec16c {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        unsafe {
            let mut aeven = _mm_slli_epi16(self.xmm, 8);
            aeven = _mm_sra_epi16(aeven, _mm_cvtsi32_si128(rhs + 8));
            let aodd = _mm_sra_epi16(self.xmm, _mm_cvtsi32_si128(rhs));
            let mask = _mm_set1_epi32(0x00FF00FF);
            Self {
                xmm: selectb(mask, aeven, aodd),
            }
        }
    }
}

/// vector operator >>= : shift right arithmetic
impl ops::ShrAssign<i32> for Vec16c {
    #[inline]
    fn shr_assign(&mut self, rhs: i32) {
        *self = *self >> rhs
    }
}

/// vector operator & : bitwise and
impl ops::BitAnd for Vec16c {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            xmm: unsafe { _mm_add_epi8(self.xmm, other.xmm) },
        }
    }
}

/// vector operator &= : bitwise and
impl ops::BitAndAssign for Vec16c {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

/// vector operator == : equality
impl PartialEq for Vec16c {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let neq = _mm_xor_si128(self.xmm, other.xmm);
            _mm_test_all_zeros(neq, neq) > 0
        }
    }
}

///# Safety: Each byte in s must be either 0 (false) or -1 (true). No other values are allowed.
///
/// Select between two operands. Corresponds to this pseudocode:
/// for (int i = 0; i < 16; i++) result[i] = s[i] ? a[i] : b[i];
#[inline]
pub fn select(s: Vec16bc, a: Vec16c, b: Vec16c) -> Vec16c {
    unsafe {
        Vec16c {
            xmm: selectb(s.xmm, a.xmm, b.xmm),
        }
    }
}

/// Conditional add: For all vector elements i: result[i] = f[i] ? (a[i] + b[i]) : a[i]
#[inline]
pub fn if_add(f: Vec16bc, a: Vec16c, b: Vec16c) -> Vec16c {
    a + ((Vec16c::from(f)) & b)
}

/// Conditional sub: For all vector elements i: result[i] = f[i] ? (a[i] - b[i]) : a[i]
#[inline]
pub fn if_sub(f: Vec16bc, a: Vec16c, b: Vec16c) -> Vec16c {
    a - ((Vec16c::from(f)) & b)
}

/// Conditional mul: For all vector elements i: result[i] = f[i] ? (a[i] * b[i]) : a[i]
#[inline]
pub fn if_mul(f: Vec16bc, a: Vec16c, b: Vec16c) -> Vec16c {
    select(f, a * b, a)
}

/// Horizontal add: Calculates the sum of all vector elements. Overflow will wrap around
#[inline]
pub fn horizontal_add(a: Vec16c) -> i32 {
    unsafe {
        let sum1 = _mm_sad_epu8(a.xmm, _mm_setzero_si128());
        let sum2 = _mm_unpackhi_epi64(sum1, sum1);
        let sum3 = _mm_add_epi16(sum1, sum2);
        _mm_cvtsi128_si32(sum3) as i32
    }
}

/// Horizontal add extended: Calculates the sum of all vector elements.
/// Each element is sign-extended before addition to avoid overflow
#[inline]
pub fn horizontal_add_x(a: Vec16c) -> i32 {
    unsafe {
        let mut aeven = _mm_slli_epi16(a.xmm, 8);
        aeven = _mm_srai_epi16(aeven, 8);
        let aodd = _mm_srai_epi16(a.xmm, 8);
        let sum1 = _mm_add_epi16(aeven, aodd);
        let sum2 = _mm_add_epi16(sum1, _mm_unpackhi_epi64(sum1, sum1));
        let sum3 = _mm_add_epi16(sum2, _mm_shuffle_epi32(sum2, 1));
        let sum4 = _mm_add_epi16(sum3, _mm_shufflelo_epi16(sum3, 1));
        _mm_cvtsi128_si32(sum4) as i32
    }
}

/// function add_saturated: add element by element, signed with saturatio
#[inline]
pub fn add_saturated(a: Vec16c, b: Vec16c) -> Vec16c {
    unsafe {
        Vec16c {
            xmm: _mm_adds_epi8(a.xmm, b.xmm),
        }
    }
}

/// function sub_saturated: subtract element by element, signed with saturation
#[inline]
pub fn sub_saturated(a: Vec16c, b: Vec16c) -> Vec16c {
    unsafe {
        Vec16c {
            xmm: _mm_subs_epi8(a.xmm, b.xmm),
        }
    }
}

/// function max: a > b ? a : b
#[inline]
pub fn max(a: Vec16c, b: Vec16c) -> Vec16c {
    unsafe {
        let signbit = _mm_set1_epi32(0x80808080u32 as i32);
        let a1 = _mm_xor_si128(a.xmm, signbit);
        let b1 = _mm_xor_si128(b.xmm, signbit);
        let m1 = _mm_max_epu8(a1, b1);
        Vec16c {
            xmm: _mm_xor_si128(m1, signbit),
        }
    }
}

/// function min: a < b ? a : b
#[inline]
pub fn min(a: Vec16c, b: Vec16c) -> Vec16c {
    unsafe {
        let signbit = _mm_set1_epi32(0x80808080u32 as i32);
        let a1 = _mm_xor_si128(a.xmm, signbit);
        let b1 = _mm_xor_si128(b.xmm, signbit);
        let m1 = _mm_min_epu8(a1, b1);
        Vec16c {
            xmm: _mm_xor_si128(m1, signbit),
        }
    }
}

/// function abs: a >= 0 ? a : -a
#[inline]
pub fn abs(a: Vec16c) -> Vec16c {
    unsafe {
        let nega = _mm_sub_epi8(_mm_setzero_si128(), a.xmm);
        Vec16c {
            xmm: _mm_min_epu8(a.xmm, nega),
        }
    }
}

/// function abs_saturated: same as abs, saturate if overflow
#[inline]
pub fn abs_saturated(a: Vec16c) -> Vec16c {
    unsafe {
        let absa = abs(a);
        let overfl = _mm_cmpgt_epi8(_mm_setzero_si128(), absa.xmm);
        Vec16c {
            xmm: _mm_add_epi8(absa.xmm, overfl),
        }
    }
}

/// function rotate_left: rotate each element left by b bits
/// Use negative count to rotate right
#[inline]
pub fn rotate_left(a: Vec16c, b: i32) -> Vec16c {
    unsafe {
        let mask = (0xFFu32 << b) as i8;
        let m = _mm_set1_epi8(mask);
        let bb = _mm_cvtsi32_si128(b & 7);
        let mbb = _mm_cvtsi32_si128((-b) & 7);
        let mut left = _mm_sll_epi16(a.xmm, bb);
        let mut right = _mm_srl_epi16(a.xmm, mbb);
        left = _mm_and_si128(m, left);
        right = _mm_andnot_si128(m, right);
        Vec16c {
            xmm: _mm_or_si128(left, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
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
        }
    }

    #[test]
    fn test_vec16c() {
        let mut rng = rand::thread_rng();

        unsafe {
            let mut a16 = Vec16c::new();
            let arr1: [i8; 16] = core::array::from_fn(|_| rng.gen_range(0..50));
            let b16 = Vec16c::set_values(arr1);
            a16.load(&arr1 as *const [i8] as *const i8);
            println!("{:?}, {:?}", a16.xmm, b16.xmm);
            assert_eq!(a16, b16);
            assert_eq!(a16 & b16, a16 & b16);
        }
    }
}
