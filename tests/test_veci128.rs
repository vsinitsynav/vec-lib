use std::arch::x86_64::*;
use vec_lib::vectori128::{horizontal_and, horizontal_or, Vec128b};

#[test]
fn test_vec128b() {
    unsafe {
        let mut arr: [i32; 4] = [0; 4];
        let mut a128 = Vec128b::new();
        a128.store(&mut arr as *mut i32 as *mut __m128i);
        assert_eq!(false, horizontal_or(a128));

        arr[0] = 1;
        a128.load(&arr as *const i32 as *const __m128i);
        assert_eq!(true, horizontal_or(a128));
        assert_eq!(false, horizontal_and(a128));

        a128 ^= a128;
        assert_eq!(false, horizontal_or(a128));
    }
}
