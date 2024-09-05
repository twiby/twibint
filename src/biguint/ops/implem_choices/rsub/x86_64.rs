use std::arch::asm;

/// Performs a part of the subtraction. Returns a tuple containing the carry state
/// and the number of digits currently sbtracted
pub(super) unsafe fn schoolbook_rsub_assign_x86_64(
    rhs: *mut u64,
    lhs: *const u64,
    mut size: usize,
) -> (bool, usize) {
    size /= 5;
    if size == 0 {
        return (false, 0);
    }

    let mut c: u8;
    let mut idx = 0;

    asm!(
        // Clear carry flag
        "clc",

        "3:",

        // Copy a in registers
        "mov {a_tmp1}, qword ptr [{a:r} + 8*{idx:r}]",
        "mov {a_tmp2}, qword ptr [{a:r} + 8*{idx:r} + 8]",
        "mov {a_tmp3}, qword ptr [{a:r} + 8*{idx:r} + 16]",
        "mov {a_tmp4}, qword ptr [{a:r} + 8*{idx:r} + 24]",
        "mov {a_tmp5}, qword ptr [{a:r} + 8*{idx:r} + 32]",

        // Copy b in registers
        "mov {b_tmp1}, qword ptr [{b:r} + 8*{idx:r}]",
        "mov {b_tmp2}, qword ptr [{b:r} + 8*{idx:r} + 8]",
        "mov {b_tmp3}, qword ptr [{b:r} + 8*{idx:r} + 16]",
        "mov {b_tmp4}, qword ptr [{b:r} + 8*{idx:r} + 24]",
        "mov {b_tmp5}, qword ptr [{b:r} + 8*{idx:r} + 32]",

        // Perform the subtraction
        "sbb {b_tmp1}, {a_tmp1}",
        "sbb {b_tmp2}, {a_tmp2}",
        "sbb {b_tmp3}, {a_tmp3}",
        "sbb {b_tmp4}, {a_tmp4}",
        "sbb {b_tmp5}, {a_tmp5}",

        // Copy the return values
        "mov qword ptr [{a:r} + 8*{idx:r}], {b_tmp1}",
        "mov qword ptr [{a:r} + 8*{idx:r} + 8], {b_tmp2}",
        "mov qword ptr [{a:r} + 8*{idx:r} + 16], {b_tmp3}",
        "mov qword ptr [{a:r} + 8*{idx:r} + 24], {b_tmp4}",
        "mov qword ptr [{a:r} + 8*{idx:r} + 32], {b_tmp5}",

        // Increment loop counter
        "inc {idx:r}",
        "inc {idx:r}",
        "inc {idx:r}",
        "inc {idx:r}",
        "inc {idx:r}",
        "dec {size:r}",
        "jnz 3b",

        // Output carry flag and clear
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

    (c > 0, idx)
}
