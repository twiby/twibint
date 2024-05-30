mod add;
mod mul;
mod sub;

pub(super) use add::add_assign;
pub(super) use mul::mul;
pub(super) use sub::sub_assign;

#[cfg(feature = "unsafe")]
pub(crate) fn u32_ptrs_aligned(a: *const u32, b: *const u32) -> bool {
    a.align_offset(std::mem::align_of::<u64>()) == 0 && b.align_offset(std::mem::align_of::<u64>()) == 0
}
