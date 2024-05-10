use crate::traits::Digit;
use crate::traits::ToPtr;
use std::arch::asm;

/// Performs a part of the subtraction. Returns a tuple containing the carry state
/// and the number of digits currently sbtracted
fn schoolbook_sub_assign_x64_64(rhs: *mut u64, lhs: *const u64, size: usize) -> (bool, usize) {
    let mut c = 0u8;
    let mut idx = 0;

    unsafe {
        asm!(
            "3:",

            // Copy a in registers
            "mov {a_tmp1}, qword ptr [{a} + 8*{idx}]",
            "mov {a_tmp2}, qword ptr [{a} + 8*{idx} + 8]",
            "mov {a_tmp3}, qword ptr [{a} + 8*{idx} + 16]",
            "mov {a_tmp4}, qword ptr [{a} + 8*{idx} + 24]",
            "mov {a_tmp5}, qword ptr [{a} + 8*{idx} + 32]",

            // Copy b in registers
            "mov {b_tmp1}, qword ptr [{b} + 8*{idx}]",
            "mov {b_tmp2}, qword ptr [{b} + 8*{idx} + 8]",
            "mov {b_tmp3}, qword ptr [{b} + 8*{idx} + 16]",
            "mov {b_tmp4}, qword ptr [{b} + 8*{idx} + 24]",
            "mov {b_tmp5}, qword ptr [{b} + 8*{idx} + 32]",

            // Set the carry flag if there was a previous carry
            "cmp {c}, 0",
            "jle 2f",
            "stc",

            // Perform the addition
            "2:",
            "sbb {a_tmp1}, {b_tmp1}",
            "sbb {a_tmp2}, {b_tmp2}",
            "sbb {a_tmp3}, {b_tmp3}",
            "sbb {a_tmp4}, {b_tmp4}",
            "sbb {a_tmp5}, {b_tmp5}",

            // Copy the return values
            "mov qword ptr [{a} + 8*{idx}], {a_tmp1}",
            "mov qword ptr [{a} + 8*{idx} + 8], {a_tmp2}",
            "mov qword ptr [{a} + 8*{idx} + 16], {a_tmp3}",
            "mov qword ptr [{a} + 8*{idx} + 24], {a_tmp4}",
            "mov qword ptr [{a} + 8*{idx} + 32], {a_tmp5}",

            // Output and clear the carry flag
            "setc {c}",
            "clc",

            // Increment loop counter
            "add {idx}, 5",
            "cmp {idx}, {size}",
            "jle 3b",

            size = in(reg) size,
            a = in(reg) rhs,
            b = in(reg) lhs,
            c = inout(reg_byte) c,
            idx = inout(reg) idx,

            a_tmp1 = out(reg) _,
            a_tmp2 = out(reg) _,
            a_tmp3 = out(reg) _,
            a_tmp4 = out(reg) _,
            a_tmp5 = out(reg) _,

            b_tmp1 = out(reg) _,
            b_tmp2 = out(reg) _,
            b_tmp3 = out(reg) _,
            b_tmp4 = out(reg) _,
            b_tmp5 = out(reg) _,

            options(nostack),
        );
    }

    (c > 0, idx)
}

/// Assumes rhs > lhs
pub(crate) fn sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) {
    #[allow(unused_mut)]
    let mut done = 0;
    #[allow(unused_mut)]
    let mut carry = false;

    #[cfg(target_arch = "x86_64")]
    'x86_u64_spec: {
        if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u64>(), lhs.to_ptr::<u64>()) {
            assert_eq!(T::NB_BITS, 64);

            let size = lhs.len().min(rhs.len());
            if size <= 5 {
                break 'x86_u64_spec;
            }
            let (c, d) = schoolbook_sub_assign_x64_64(rhs_cast, lhs_cast, size - 5);
            debug_assert!(size - d < 5);
            done += d;
            carry = c;
        }

        if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u32>(), lhs.to_ptr::<u32>()) {
            assert_eq!(T::NB_BITS, 32);
            let size = lhs.len().min(rhs.len()) / 2;
            if size <= 5 {
                break 'x86_u64_spec;
            }
            let (c, d) = schoolbook_sub_assign_x64_64(rhs_cast.cast(), lhs_cast.cast(), size - 5);
            debug_assert!(size - d < 5);
            done += d * 2;
            carry = c;
        }
    }

    schoolbook_sub_assign(&mut rhs[done..], &lhs[done..], carry);
}

fn schoolbook_sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T], mut carry: bool) {
    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        (*a, partial_carry_1) = a.overflowing_sub(*b);
        (*a, partial_carry_2) = a.overflowing_sub(T::from(carry));
        carry = partial_carry_1 | partial_carry_2;
    }

    for val in rhs.iter_mut().skip(lhs.len()) {
        (*val, carry) = val.overflowing_sub(T::from(carry));
    }
}
