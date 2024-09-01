mod add;
mod mul;
mod rsub;
mod sub;

pub(crate) use add::add_assign;
pub(crate) use mul::mul;
pub(crate) use rsub::rsub_assign;
pub(crate) use sub::sub_assign;

#[cfg(feature = "unsafe")]
pub(crate) fn u32_ptrs_aligned(a: *const u32, b: *const u32) -> bool {
    a.align_offset(std::mem::align_of::<u64>()) == 0
        && b.align_offset(std::mem::align_of::<u64>()) == 0
}
