pub(super) fn schoolbook_add_assign(rhs: &mut [u32], lhs: &[u32]) -> bool {
    let mut carry = 0u64;

    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        let full = (*a as u64) + (*b as u64) + carry;
        (*a, carry) = (full as u32, full >> 32);
    }

    // Potential carry propagation
    for val in rhs.iter_mut().skip(lhs.len()) {
        let full = (*val as u64) + carry;
        (*val, carry) = (full as u32, full >> 32);
    }

    carry != 0
}
