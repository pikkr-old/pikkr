//! Pikkr
//!
//! This module provides functions using SIMD instructions for Pikkr.

extern crate x86intrin;

use x86intrin::{mm256_setr_epi8, mm256_cmpeq_epi8, mm256_movemask_epi8};

/// Builds a structural character bitmap and returns it.
#[inline(never)]
#[no_mangle]
pub fn build_structural_character_bitmap(e63: i8, e62: i8, e61: i8, e60: i8, e59: i8, e58: i8, e57: i8, e56: i8,
                                         e55: i8, e54: i8, e53: i8, e52: i8, e51: i8, e50: i8, e49: i8, e48: i8,
                                         e47: i8, e46: i8, e45: i8, e44: i8, e43: i8, e42: i8, e41: i8, e40: i8,
                                         e39: i8, e38: i8, e37: i8, e36: i8, e35: i8, e34: i8, e33: i8, e32: i8,
                                         e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8, e24: i8,
                                         e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8,
                                         e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8,
                                         e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8,
                                         c: i8) -> i64 {
    let i1 = mm256_setr_epi8(e63, e62, e61, e60, e59, e58, e57, e56,
                             e55, e54, e53, e52, e51, e50, e49, e48,
                             e47, e46, e45, e44, e43, e42, e41, e40,
                             e39, e38, e37, e36, e35, e34, e33, e32);
    let i2 = mm256_setr_epi8(e31, e30, e29, e28, e27, e26, e25, e24,
                             e23, e22, e21, e20, e19, e18, e17, e16,
                             e15, e14, e13, e12, e11, e10, e9, e8,
                             e7, e6, e5, e4, e3, e2, e1, e0);
    let m = mm256_setr_epi8(c, c, c, c, c, c, c, c,
                            c, c, c, c, c, c, c, c,
                            c, c, c, c, c, c, c, c,
                            c, c, c, c, c, c, c, c);
    let r1 = mm256_movemask_epi8(mm256_cmpeq_epi8(i1, m));
    let r2 = mm256_movemask_epi8(mm256_cmpeq_epi8(i2, m));
    (((r2 as u32) as i64) << 32) | ((r1 as u32) as i64)
}
