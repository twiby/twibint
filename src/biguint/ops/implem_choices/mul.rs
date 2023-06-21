use core::cmp::max;

pub(super) fn schoolbook_add_assign_mul(ret: &mut [u32], rhs: &[u32], lhs: &[u32]) {
    for (idx_1, b) in rhs.iter().enumerate() {
        let mut carry = 0u64;

        for (a, r) in lhs.iter().zip(&mut ret[idx_1..]) {
            let full = (*a as u64) * (*b as u64) + (*r as u64) + carry;
            (*r, carry) = (full as u32, (full >> 32));
        }

        ret[idx_1 + lhs.len()] = carry as u32
    }
}

pub(super) fn karatsuba<const THRESHOLD: usize>(rhs: &[u32], lhs: &[u32]) -> Vec<u32> {
    let target_length = max(rhs.len(), lhs.len()).next_power_of_two();
    assert!(target_length < usize::MAX >> 1);

    let mut x = rhs.to_vec();
    let mut y = lhs.to_vec();
    x.resize(target_length, 0);
    y.resize(target_length, 0);

    let mut ret = vec![0u32; target_length << 1];
    _karatsuba::<THRESHOLD>(&mut ret, &x, &y);
    ret
}
fn _karatsuba<const THRESHOLD: usize>(ret: &mut [u32], rhs: &[u32], lhs: &[u32]) {
    assert!(rhs.len() == lhs.len());
    assert!(rhs.len().is_power_of_two());
    assert_eq!(ret.len(), 2 * rhs.len());
    for i in 0..ret.len() {
        assert_eq!(ret[i], 0u32);
    }

    // Early exit
    if rhs.len() < THRESHOLD {
        schoolbook_add_assign_mul(ret, rhs, lhs);
        return;
    }
    let size = rhs.len();
    let half_size = size >> 1;

    let x0: &[u32] = &rhs[..half_size];
    let y0: &[u32] = &lhs[..half_size];
    let x1: &[u32] = &rhs[half_size..];
    let y1: &[u32] = &lhs[half_size..];

    // z0 and z2
    _karatsuba::<THRESHOLD>(&mut ret[..size], &x0, &y0);
    _karatsuba::<THRESHOLD>(&mut ret[size..], &x1, &y1);

    // compute z1 in a separate buffer
    let mut x_temp = x0.to_vec();
    let mut y_temp = y0.to_vec();
    let x_carry = super::add::schoolbook_add_assign(&mut x_temp, &x1);
    let y_carry = super::add::schoolbook_add_assign(&mut y_temp, &y1);
    let mut z1 = vec![0u32; size + 1];
    _karatsuba::<THRESHOLD>(&mut z1[..size], &x_temp, &y_temp);
    if x_carry {
        super::add::schoolbook_add_assign(&mut z1[half_size..], &y_temp);
    }
    if y_carry {
        super::add::schoolbook_add_assign(&mut z1[half_size..], &x_temp);
    }
    if x_carry && y_carry {
        z1[size] += 1;
    }

    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    let mut partial_carry_3: bool;
    let mut carry = 0u32;
    for i in 0..size {
        (z1[i], partial_carry_1) = z1[i].overflowing_sub(ret[i + size]);
        (z1[i], partial_carry_2) = z1[i].overflowing_sub(ret[i]);
        (z1[i], partial_carry_3) = z1[i].overflowing_sub(carry);
        carry = (partial_carry_1 as u32) + (partial_carry_2 as u32) + (partial_carry_3 as u32);
    }
    (z1[size], _) = z1[size].overflowing_sub(carry);

    // add z1
    super::add::schoolbook_add_assign(&mut ret[half_size..], &z1);
}
