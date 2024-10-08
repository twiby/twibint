mod add;
mod div;
mod mul;
mod rsub;
mod sub;

pub(crate) use add::add_assign;
pub(crate) use div::div;
pub(crate) use mul::mul;
pub(crate) use mul::mul_assign_digit;
pub(crate) use rsub::rsub_assign;
pub(crate) use sub::sub_assign;

#[cfg(feature = "unsafe")]
pub(crate) fn u32_ptr_aligned(a: *mut u32) -> bool {
    a.align_offset(std::mem::align_of::<u64>()) == 0
}

#[cfg(feature = "unsafe")]
pub(crate) fn u32_ptrs_aligned_2(a: *const u32, b: *const u32) -> bool {
    a.align_offset(std::mem::align_of::<u64>()) == 0
        && b.align_offset(std::mem::align_of::<u64>()) == 0
}

#[cfg(feature = "unsafe")]
pub(crate) fn u32_ptrs_aligned_3(a: *mut u32, b: *const u32, c: *const u32) -> bool {
    a.align_offset(std::mem::align_of::<u64>()) == 0
        && b.align_offset(std::mem::align_of::<u64>()) == 0
        && c.align_offset(std::mem::align_of::<u64>()) == 0
}
