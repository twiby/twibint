use std::arch::asm;

/// Performs a part of the subtraction. Returns a tuple containing the carry state
/// and the number of digits currently sbtracted
#[cfg(target_arch = "x86_64")]
pub(super) fn schoolbook_sub_assign_x64_64(
    rhs: *mut u64,
    lhs: *const u64,
    mut size: usize,
) -> (bool, usize) {
    if size <= 5 {
        return (false, 0);
    }
    size -= 5;
    size /= 5;
    size += 1;

    let mut c: u8;
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

            // Perform the subtraction
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

            // Increment loop counter
            "inc {idx}",
            "inc {idx}",
            "inc {idx}",
            "inc {idx}",
            "inc {idx}",
            "dec {size}",
            "jnz 3b",

            "setc {c}",
            "clc",

            size = in(reg) size,
            a = in(reg) rhs,
            b = in(reg) lhs,
            c = lateout(reg_byte) c,
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
