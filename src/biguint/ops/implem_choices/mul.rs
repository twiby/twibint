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
    let mut buff = vec![0u32; target_length << 1];
    _karatsuba::<THRESHOLD>(&mut ret, &x, &y, &mut buff);
    ret
}
fn _karatsuba<const THRESHOLD: usize>(ret: &mut [u32], rhs: &[u32], lhs: &[u32], buff: &mut [u32]) {
    debug_assert!(rhs.len() == lhs.len());
    debug_assert!(rhs.len().is_power_of_two());
    debug_assert_eq!(ret.len(), 2 * rhs.len());
    debug_assert_eq!(buff.len(), 2 * rhs.len());
    for i in 0..ret.len() {
        debug_assert_eq!(ret[i], 0u32);
    }
    for i in 0..buff.len() {
        debug_assert_eq!(buff[i], 0u32);
    }

    let size = rhs.len();
    let half_size = size >> 1;

    // Early exit
    if size < THRESHOLD {
        schoolbook_add_assign_mul(ret, rhs, lhs);
        return;
    }

    let (x0, x1) = rhs.split_at(half_size);
    let (y0, y1) = lhs.split_at(half_size);

    // Compute (x0+x1) and (y0+y1), using ret as a buffer,
    // but specifically handle their last bit
    let (x_temp, y_temp) = ret[..size].split_at_mut(half_size);
    x_temp.copy_from_slice(x0);
    y_temp.copy_from_slice(y0);
    let x_carry = super::add::schoolbook_add_assign(x_temp, x1);
    let y_carry = super::add::schoolbook_add_assign(y_temp, y1);

    // compute z1 in a separate buffer
    // but specifically handle its last bit
    let (z1, new_buff) = buff.split_at_mut(size);
    let mut z1_last_bit = 0u32;
    _karatsuba::<THRESHOLD>(&mut z1[..size], x_temp, y_temp, new_buff);
    if x_carry {
        z1_last_bit += super::add::schoolbook_add_assign(&mut z1[half_size..], &y_temp) as u32;
    }
    if y_carry {
        z1_last_bit += super::add::schoolbook_add_assign(&mut z1[half_size..], &x_temp) as u32;
    }
    z1_last_bit += (x_carry && y_carry) as u32;

    // z0 and z2
    ret[..size].fill(0);
    new_buff.fill(0);
    _karatsuba::<THRESHOLD>(&mut ret[..size], x0, y0, new_buff);
    new_buff.fill(0);
    _karatsuba::<THRESHOLD>(&mut ret[size..], x1, y1, new_buff);

    // subtract z0 and z2 from z1
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
    (z1_last_bit, _) = z1_last_bit.overflowing_sub(carry);

    // add z1
    super::add::schoolbook_add_assign(&mut ret[half_size..], z1);
    super::add::schoolbook_add_assign(&mut ret[half_size + size..], &[z1_last_bit]);
}
