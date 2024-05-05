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
fn schoolbook_add_assign_x64_64(rhs: *mut u64, lhs: *const u64, size: usize) -> (bool, usize) {
    let mut c = 0u64;
    const FACTOR: usize = 4;

    let size_div = size / FACTOR;
    for i in 0..size_div {
        let idx = i * FACTOR;

        unsafe {
            asm!(
                // Copy a in registers
                "mov {a_tmp1}, qword ptr [{a}]",
                "mov {a_tmp2}, qword ptr [{a} + 8]",
                "mov {a_tmp3}, qword ptr [{a} + 16]",
                "mov {a_tmp4}, qword ptr [{a} + 24]",

                // Copy b in registers
                "mov {b_tmp1}, qword ptr [{b}]",
                "mov {b_tmp2}, qword ptr [{b} + 8]",
                "mov {b_tmp3}, qword ptr [{b} + 16]",
                "mov {b_tmp4}, qword ptr [{b} + 24]",

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
                "mov qword ptr [{a}], {a_tmp1}",
                "mov qword ptr [{a} + 8], {a_tmp2}",
                "mov qword ptr [{a} + 16], {a_tmp3}",
                "mov qword ptr [{a} + 24], {a_tmp4}",

                // Output the carry flag
                "setc dl",
                "movzx {c}, dl",
                "clc",
                a = in(reg) rhs.add(idx), 
                b = in(reg) lhs.add(idx), 
                c = inout(reg) c,

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

    (c > 0, size_div * FACTOR)
}

pub(super) fn schoolbook_add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    #[allow(unused_mut)]
    let mut done = 0;
    let mut carry = T::ZERO;

    #[cfg(target_arch="x86_64")]
    'x86_spec: {
        let size = lhs.len().min(rhs.len());
        // Avboid specialization overhead for small sizes
        if size < 256 {
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
            if c {
                carry = T::ONE;
            }
        }
        
    }

    for (a, b) in rhs[done..].iter_mut().zip(lhs[done..].iter()) {
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
