use std::arch::asm;

// TODO: some of these may be simd-able

pub(super) unsafe fn single_digit_add_assign_mul_x86_64(
    ret: *mut u64,
    rhs: *const u64,
    b: u64,
    mut len: usize,
) -> (u64, usize) {
    if len <= 4 {
        return (0, 0);
    }
    len -= 4;

    let mut carry = 0;
    let mut idx = 0;

    asm!(
        "3:",

        // Get data
        "mov rax, qword ptr[{a:r} + 8*{i:r}]",
        "mov {temp_a_2}, qword ptr [{a:r} + 8*{i:r} + 8]",
        "mov {temp_a_3}, qword ptr [{a:r} + 8*{i:r} + 16]",
        "mov {temp_a_4}, qword ptr [{a:r} + 8*{i:r} + 24]",
        "mov {temp_r_1}, qword ptr[{r:r} + 8*{i:r}]",
        "mov {temp_r_2}, qword ptr[{r:r} + 8*{i:r} + 8]",
        "mov {temp_r_3}, qword ptr[{r:r} + 8*{i:r} + 16]",
        "mov {temp_r_4}, qword ptr[{r:r} + 8*{i:r} + 24]",

        // Multiply
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_1}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r:r} + 8*{i:r}], rax",

        // Next mul
        "mov rax, {temp_a_2}",
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_2}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r:r} + 8*{i:r} + 8], rax",

        // Next mul
        "mov rax, {temp_a_3}",
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_3}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r:r} + 8*{i:r} + 16], rax",

        // Next mul
        "mov rax, {temp_a_4}",
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_4}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r:r} + 8*{i:r} + 24], rax",

        // Increment loop counter
        "add {i:r}, 4",
        "cmp {i:r}, {len:r}",
        "jle 3b",

        out("rax") _,
        out("rdx") _,
        b = in(reg) b,
        a = in(reg) rhs,
        r = in(reg) ret,
        c = inout(reg) carry,
        i = inout(reg) idx,
        len = in(reg) len,

        temp_r_1 = out(reg) _,
        temp_r_2 = out(reg) _,
        temp_r_3 = out(reg) _,
        temp_r_4 = out(reg) _,
        temp_a_2 = out(reg) _,
        temp_a_3 = out(reg) _,
        temp_a_4 = out(reg) _,
    );

    (carry, idx)
}

#[cfg(test)]
mod tests {
    #[test]
    fn full_carrying_mul() {
        let a = vec![u64::MAX; 16];
        let b = u64::MAX;
        let mut c = vec![u64::MAX; 16];

        unsafe {
            let (carry, done) =
                super::single_digit_add_assign_mul_x86_64(c.as_mut_ptr(), a.as_ptr(), b, 16);
            c.push(carry);
            assert_eq!(done, 16);
        }

        let mut shoud_get = vec![u64::MAX; 16];
        shoud_get.insert(0, 0u64);
        assert_eq!(c, shoud_get);
    }
}
