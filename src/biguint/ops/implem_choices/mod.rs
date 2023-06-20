fn schoolbook_add_assign_mul(ret: &mut [u32], rhs: &[u32], lhs: &[u32]) {
    for (idx_1, b) in rhs.iter().enumerate() {
        let mut carry = 0u64;

        for (a, r) in lhs.iter().zip(&mut ret[idx_1..]) {
            let full = (*a as u64) * (*b as u64) + (*r as u64) + carry;
            (*r, carry) = (full as u32, (full >> 32));
        }

        ret[idx_1 + lhs.len()] = carry as u32;
    }
}

/// Current implementation of multiplication
pub(super) fn mul(rhs: &[u32], lhs: &[u32]) -> Vec<u32> {
    let mut ret = vec![0u32; rhs.len() + lhs.len()];
    schoolbook_add_assign_mul(&mut ret, rhs, lhs);
    ret
}

/// Current implementation of add_assign, returning the carry
/// Assumes rhs has at least the size of lhs
pub(super) fn add_assign(rhs: &mut [u32], lhs: &[u32]) -> u32 {
    let mut carry = 0u64;

    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        let full = (*a as u64) + (*b as u64) + carry;
        (*a, carry) = (full as u32, full >> 32);
    }

    for val in rhs.iter_mut().skip(lhs.len()) {
        let full = (*val as u64) + carry;
        (*val, carry) = (full as u32, full >> 32);
    }

    carry as u32
}

/// Current implementation of sub_assign
/// Assumes rhs > lhs
pub(super) fn sub_assign(rhs: &mut [u32], lhs: &[u32]) {
    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    let mut carry = false;
    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        (*a, partial_carry_1) = a.overflowing_sub(*b);
        (*a, partial_carry_2) = a.overflowing_sub(carry as u32);
        carry = partial_carry_1 | partial_carry_2;
    }

    for val in rhs.iter_mut().skip(lhs.len()) {
        (*val, carry) = val.overflowing_sub(carry as u32);
    }
}
