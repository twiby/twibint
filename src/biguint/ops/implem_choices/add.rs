use std::arch::asm;

use crate::traits::{Digit, DoubleDigit, ToPtr};

// TODO: x86_64 u32
// TODO: arm u32
// TODO: arm u64
// TODO: check apple m chips for arm specialization
// TODO: generalize this for overwriting add ?
// multiplication ?

/// Performs a part of the addition. Returns a tuple containing the carry state 
/// and the number of digits currently added
fn schoolbook_add_assign_x64_64(rhs: *mut u64, lhs: *const u64, mut size: usize) -> (bool, usize) {
    if size <= 4 {
        return (false, 0);
    }
    size -= 4;

    let mut c = 0u64;

    let mut idx = 0;
    while idx < size {
        unsafe {
            asm!(
                // Copy a in registers
                "mov {a_tmp1}, qword ptr [{a} + 8*{idx}]",
                "mov {a_tmp2}, qword ptr [{a} + 8*{idx} + 8]",
                "mov {a_tmp3}, qword ptr [{a} + 8*{idx} + 16]",
                "mov {a_tmp4}, qword ptr [{a} + 8*{idx} + 24]",

                // Copy b in registers
                "mov {b_tmp1}, qword ptr [{b} + 8*{idx}]",
                "mov {b_tmp2}, qword ptr [{b} + 8*{idx} + 8]",
                "mov {b_tmp3}, qword ptr [{b} + 8*{idx} + 16]",
                "mov {b_tmp4}, qword ptr [{b} + 8*{idx} + 24]",

                // Set the carry flag if there was a previous carry
                "cmp {c}, 0",
                "jle 2f",
                "stc",

                // Perform the addition
                "2:",
                "adc {a_tmp1}, {b_tmp1}",
                "adc {a_tmp2}, {b_tmp2}",
                "adc {a_tmp3}, {b_tmp3}",
                "adc {a_tmp4}, {b_tmp4}",

                // Copy the return values
                "mov qword ptr [{a} + 8*{idx}], {a_tmp1}",
                "mov qword ptr [{a} + 8*{idx} + 8], {a_tmp2}",
                "mov qword ptr [{a} + 8*{idx} + 16], {a_tmp3}",
                "mov qword ptr [{a} + 8*{idx} + 24], {a_tmp4}",

                // Output the carry flag
                "setc dl",
                "movzx {c}, dl",
                "clc",

                // Increment loop counter
                "add {idx}, 4",

                a = in(reg) rhs, 
                b = in(reg) lhs, 
                c = inout(reg) c,
                idx = inout(reg) idx,

                a_tmp1 = out(reg) _,
                a_tmp2 = out(reg) _,
                a_tmp3 = out(reg) _,
                a_tmp4 = out(reg) _,

                b_tmp1 = out(reg) _,
                b_tmp2 = out(reg) _,
                b_tmp3 = out(reg) _,
                b_tmp4 = out(reg) _,

                options(nostack),
            );
        }
    }

    (c > 0, idx)
}

#[cfg(test)]
const SPECIALIZATION_THRESHOLD: usize = 16;

#[cfg(not(test))]
const SPECIALIZATION_THRESHOLD: usize = 256;

/// Current implementation of add_assign, returning the carry
/// Assumes rhs has at least the size of lhs
pub(crate) fn add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    #[allow(unused_mut)]
    let mut done = 0;
    #[allow(unused_mut)]
    let mut carry = false;

    #[cfg(target_arch="x86_64")]
    'x86_spec: {
        let size = lhs.len().min(rhs.len());
        // Avoid specialization overhead for small sizes
        if size < SPECIALIZATION_THRESHOLD {
            break 'x86_spec;
        }

        'u64_spec: {
            let Some(rhs_cast) = rhs.to_mut_ptr::<u64>() else { 
                break 'u64_spec;
            };

            let Some(lhs_cast) = lhs.to_ptr::<u64>() else {
                break 'u64_spec;
            };

            let (c, d) = schoolbook_add_assign_x64_64(rhs_cast, lhs_cast, size);
            done += d;
            carry = c;
        }
    }

    schoolbook_add_assign(&mut rhs[done..], &lhs[done..], carry)
}

fn schoolbook_add_assign<T: Digit>(rhs: &mut [T], lhs: &[T], carry: bool) -> bool {
    let mut carry = if carry {
        T::ONE
    } else {
        T::ZERO
    };

    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        let full = a.to_double() + b.to_double() + carry.to_double();
        (*a, carry) = full.split();
    }

    // Potential carry propagation
    for val in rhs.iter_mut().skip(lhs.len()) {
        let full = val.to_double() + carry.to_double();
        (*val, carry) = full.split();
    }

    carry != T::ZERO
}
