use crate::biguint::ops::add_assign;
use crate::biguint::ops::mul;
#[cfg(test)]
use crate::biguint::ops::rsub_assign;
use crate::biguint::ops::sub_assign;
use crate::biguint::ord;
use crate::traits::Digit;
use crate::traits::DivisionResult;
#[cfg(test)]
use crate::traits::DoubleDigit;
use crate::BigInt;
use std::cmp::Ordering;

use crate::BigUint;

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
enum Quotient3By2<T: Digit> {
    Single(T),
    MaxPlusOne,
    MaxPlusTwo,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
enum Quotient4By2<T: Digit> {
    Single([T; 2]),
    MaxPlusOnePlus([T; 2]),
}

#[cfg(test)]
impl<T: Digit> Quotient3By2<T> {
    fn dec(&mut self) {
        *self = match self {
            Self::Single(n) => Self::Single(*n - T::ONE),
            Self::MaxPlusOne => Self::Single(T::MAX),
            Self::MaxPlusTwo => Self::MaxPlusOne,
        };
    }

    fn mul(&self, a: &[T; 2], ret: &mut [T; 4]) {
        match self {
            Self::Single(n) => mul(&mut ret[..3], a, &[*n]),
            Self::MaxPlusOne => mul(ret, a, &[T::ZERO, T::ONE]),
            Self::MaxPlusTwo => mul(ret, a, &[T::ONE, T::ONE]),
        }
    }
}

#[cfg(test)]
impl<T: Digit> Quotient4By2<T> {
    fn pack(lsb: Quotient3By2<T>, msb: Quotient3By2<T>) -> Self {
        match (lsb, msb) {
            (Quotient3By2::Single(lsb), Quotient3By2::Single(msb)) => Self::Single([lsb, msb]),
            (Quotient3By2::Single(lsb), Quotient3By2::MaxPlusOne) => {
                Self::MaxPlusOnePlus([lsb, T::ZERO])
            }
            (Quotient3By2::Single(lsb), Quotient3By2::MaxPlusTwo) => {
                Self::MaxPlusOnePlus([lsb, T::ONE])
            }
            // (Self::MaxPlusOne, Self::Single(msb)) => [T::ZERO, msb + T::ONE],
            // (Self::MaxPlusTwo, Self::Single(msb)) => [T::ONE, msb + T::ONE],
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
impl<T: DoubleDigit> From<T> for Quotient3By2<T::Single> {
    fn from(val: T) -> Quotient3By2<T::Single> {
        let (lsb, msb) = val.split();
        if msb > T::Single::ZERO {
            debug_assert!(msb == T::Single::ONE);
            debug_assert!(lsb <= T::Single::ONE);
            if lsb == T::Single::ZERO {
                Self::MaxPlusOne
            } else if lsb == T::Single::ONE {
                Self::MaxPlusTwo
            } else {
                unreachable!()
            }
        } else {
            Self::Single(lsb)
        }
    }
}

#[cfg(test)]
impl<T: Digit> Into<BigUint<T>> for Quotient3By2<T> {
    fn into(self) -> BigUint<T> {
        match self {
            Self::Single(n) => BigUint::new(n),
            Self::MaxPlusOne => BigUint::new(T::MAX) + T::ONE,
            Self::MaxPlusTwo => BigUint::new(T::MAX) + T::ONE + T::ONE,
        }
    }
}

#[cfg(test)]
impl<T: Digit> Into<BigUint<T>> for Quotient4By2<T> {
    fn into(self) -> BigUint<T> {
        match self {
            Self::Single(arr) => BigUint::from(arr.to_vec()),
            Self::MaxPlusOnePlus(arr) => BigUint::from([arr[0], arr[1], T::ONE].to_vec()),
        }
    }
}

/// A lot of assumtions here
#[cfg(test)]
fn div_3s_by_2s<T: Digit>(a: &[T; 3], b: &[T; 2], r: &mut [T; 4]) -> Quotient3By2<T> {
    debug_assert!(b[1] > T::ZERO);
    debug_assert!(b[1] > (T::MAX >> 1));
    debug_assert!(a[2] <= b[1]);

    let mut q = if a[2] == b[1] {
        Quotient3By2::<T>::MaxPlusTwo
    } else {
        debug_assert_ne!(a[2], b[1]);
        let aa = T::Double::pack(&a[1..]);
        Quotient3By2::from(aa / b[1].to_double())
    };

    q.mul(b, r);
    debug_assert_eq!(r[3], T::ZERO);
    let r = &mut r[..3];

    let mut counter = 0;
    while ord(r, a) == Ordering::Greater {
        counter += 1;
        q.dec();
        sub_assign(r, b);
    }
    debug_assert!(counter <= 3);
    rsub_assign(r, a, 3);

    q
}

#[cfg(test)]
fn div_4s_by_2s<T: Digit>(
    a: &[T; 4],
    b: &[T; 2],
    r: &mut [T; 4],
    buff: &mut [T; 2],
) -> Quotient4By2<T> {
    debug_assert!(b[1] > (T::MAX >> 1));
    if a[3] > b[1] {
        unimplemented!();
    }
    let q1 = div_3s_by_2s(&[a[1], a[2], a[3]], b, r);
    buff[0] = r[0];
    buff[1] = r[1];
    let q2 = div_3s_by_2s(&[a[0], buff[0], buff[1]], b, r);
    Quotient4By2::pack(q2, q1)
}

fn div_3n_by_2n<T: Digit>(n: &[T], d: &[T], q: &mut [T], r: &mut BigInt<T>, buff: &mut [T]) {
    let size = d.len();
    debug_assert_eq!(size % 2, 0);
    debug_assert_eq!(n.len(), 3 * (size / 2));
    debug_assert_eq!(q.len(), size / 2);
    debug_assert!(d[size - 1] > (T::MAX >> 1));
    r.sign = true;

    match ord(&n[size..], &d[size / 2..]) {
        Ordering::Less => {
            div_4n_by_2n(&n[size / 2..], &d[size / 2..], q, r, buff);
            *r <<= size / 2 * T::NB_BITS;
            r.uint.val.resize(3 * size / 2, T::ZERO);
        }
        _ => {
            q.fill(T::MAX);
            r.uint.val.resize(3 * size / 2, T::ZERO);
            r.uint.val[..size / 2].fill(T::ZERO);
            r.uint.val[size / 2..].copy_from_slice(&n[size / 2..]);
            add_assign(&mut r.uint.val[size / 2..], &d[size / 2..]);
            let remaining = sub_assign(&mut r.uint.val[size..], &d[size / 2..]);
            debug_assert!(!remaining);
        }
    };

    let buff = &mut buff[..size];
    mul(buff, q, &d[..size / 2]);
    r.add_assign(true, &n[..size / 2]);
    let buff_len = buff.len() - buff.iter().rev().take_while(|&&n| n == T::ZERO).count();
    r.sub_assign(true, &buff[..buff_len]);

    while r.is_sign_negative() {
        r.add_assign(true, d);
        sub_assign(q, &[T::ONE]);
    }
}

const RECURSION_THRESHOLD: usize = 1;

fn div_4n_by_2n<T: Digit>(n: &[T], d: &[T], q: &mut [T], r: &mut BigInt<T>, buff: &mut [T]) {
    let size = d.len();
    debug_assert_eq!(n.len(), size * 2);
    debug_assert_eq!(q.len(), size);
    debug_assert_eq!(buff.len(), size * 3);
    debug_assert!(d[size - 1] > (T::MAX >> 1));
    debug_assert!(
        (BigUint::from(d.to_vec()) << (size * T::NB_BITS)) > BigUint::from(n.to_vec()),
        "quotient won't fit !"
    );

    if size <= RECURSION_THRESHOLD || size % 2 != 0 {
        r.uint.val.resize(size, T::ZERO);

        if size == 1 {
            // Single digit case
            r.uint.val.resize(size, T::ZERO);
            let rr = super::schoolbook_div_single_digit(n, d[0], q).unwrap();
            r.uint.val[0] = rr;
        } else {
            // Normal division
            super::schoolbook_div(n, d, q, &mut r.uint.val).unwrap();
        }

        return;
    }

    let (new_n, buff) = buff.split_at_mut(3 * size / 2);

    div_3n_by_2n(&n[size / 2..], d, &mut q[size / 2..], r, buff);

    new_n[..size / 2].copy_from_slice(&n[..size / 2]);
    new_n[size / 2..size / 2 + r.uint.val.len()].copy_from_slice(&r.uint.val);
    new_n[size / 2 + r.uint.val.len()..].fill(T::ZERO);
    div_3n_by_2n(&new_n, d, &mut q[..size / 2], r, buff);
}

pub(crate) fn rem_div<T: Digit>(
    n: &BigUint<T>,
    d: &BigUint<T>,
) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
    if d.val.len() <= RECURSION_THRESHOLD {
        return super::newton_raphson::rem_div(n, d);
    }

    // Compute the target size of d, which the size of "meta blocks", acting as single
    // digits in a very high level schoolbook division
    let nb_blocks = (d.val.len() / RECURSION_THRESHOLD).next_power_of_two();
    let mut block_size = d.val.len() / nb_blocks + 1;
    if d.val.len() % nb_blocks == 0 {
        block_size -= 1;
    }
    let size = block_size * nb_blocks;

    // Shift data so that d is of the right size, and its most significant bit is one
    let d_bits = d.nb_bits();
    let size_bits = size * T::NB_BITS;
    let shift = size_bits - d_bits;
    let n_data = (n << shift).val;
    let d_data = (d << shift).val;
    debug_assert_eq!(d_data.len(), size);

    // Compute the number of "meta blocks", acting as single
    // digits in a very high level schoolbook division
    let mut n_nb_blocks = n_data.len() / size + 1;

    // Opportunity to gain one block in some cases
    // Highest block of n is not allowed to have 1 as MSB
    if n_data.len() % size == 0 && !n_data.last().map_or(false, |&n| n > (T::MAX >> 1)) {
        n_nb_blocks -= 1;
    }

    // Allocates resources
    let mut n_block = vec![T::ZERO; 2 * size];
    let mut q = vec![T::ZERO; n_nb_blocks * size];
    let mut r = BigInt::<T>::default().with_capacity(3 * size * T::NB_BITS / 2);
    let mut x2 = vec![T::ZERO; size * 3];
    let cap = r.uint.capacity();

    for tt in 0..size {
        n_block[size + tt] = *n_data
            .get((n_nb_blocks - 1) * size + tt)
            .unwrap_or(&T::ZERO);
    }
    for t in 1..n_nb_blocks {
        // We cycle through meta blocks in reverse order
        let t = n_nb_blocks - 1 - t;

        // Copy new digits
        n_block[..size].copy_from_slice(&n_data[t * size..(t + 1) * size]);

        // Perform division
        div_4n_by_2n(
            &n_block,
            &d_data,
            &mut q[t * size..(t + 1) * size],
            &mut r,
            &mut x2,
        );

        if t == 0 {
            break;
        }

        // Copy remainder in next input
        let remainder_digits = r.uint.val.len().min(size);
        n_block[size..size + remainder_digits].copy_from_slice(&r.uint.val[..remainder_digits]);
        n_block[size + remainder_digits..].fill(T::ZERO);
    }

    debug_assert_eq!(cap, r.uint.capacity());
    let r = BigUint::from(r.uint.val) >> shift;
    let q = BigUint::from(q);
    debug_assert!(&r < d);
    debug_assert_eq!(&((d * &q) + &r), n);
    Ok((q, r))
}

#[cfg(test)]
fn div<T: Digit>(n: &BigUint<T>, d: &BigUint<T>) -> DivisionResult<BigUint<T>> {
    Ok(rem_div(n, d)?.0)
}

#[cfg(test)]
#[cfg(feature = "rand")]
mod tests {
    use crate::gen_random_biguint;
    use crate::traits::Digit;
    use crate::BigUint;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;

    use typed_test_gen::test_with;

    #[test_with(u32, u64)]
    fn test_half_divider<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(3 * T::NB_BITS);
            if a.val[2] > b.val[1] {
                if b.val[1] == T::ZERO {
                    a.val[2] = T::ZERO
                } else {
                    a.val[2] = b.val[1] - T::ONE;
                }
            }

            let mut r = [T::ZERO; 4];

            let q = super::div_3s_by_2s(
                a.val[..3].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_half_divider_corner_case<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(3 * T::NB_BITS);
            a.val[2] = b.val[1];

            let mut r = [T::ZERO; 4];

            let q = super::div_3s_by_2s(
                a.val[..3].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_divider<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(4 * T::NB_BITS);
            if a.val[3] > b.val[1] {
                if b.val[1] == T::ZERO {
                    a.val[3] = T::ZERO
                } else {
                    a.val[3] = b.val[1] - T::ONE;
                }
            }

            let mut r = [T::ZERO; 4];
            let mut buff = [T::ZERO; 2];

            let q = super::div_4s_by_2s(
                a.val[..4].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
                &mut buff,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_divider_corner_case<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(4 * T::NB_BITS);
            a.val[3] = b.val[1];

            let mut r = [T::ZERO; 4];
            let mut buff = [T::ZERO; 2];

            let q = super::div_4s_by_2s(
                a.val[..4].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
                &mut buff,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_divider_corner_case_2<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(4 * T::NB_BITS);
            a.val[3] = b.val[1];
            if b.val[0] != T::MAX {
                a.val[2] = b.val[0];
            }

            let mut r = [T::ZERO; 4];
            let mut buff = [T::ZERO; 2];

            let q = super::div_4s_by_2s(
                a.val[..4].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
                &mut buff,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_divider_below<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(4 * T::NB_BITS);
            a.val[2] = T::ZERO;
            a.val[3] = T::ZERO;
            if a.val[1] > b.val[1] {
                if b.val[1] == T::ZERO {
                    a.val[1] = T::ZERO
                } else {
                    a.val[1] = b.val[1] - T::ONE;
                }
            }

            let mut r = [T::ZERO; 4];
            let mut buff = [T::ZERO; 2];

            let q = super::div_4s_by_2s(
                a.val[..4].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
                &mut buff,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());
            a.remove_leading_zeros();

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_divider_equal<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = gen_random_biguint::<T>(4 * T::NB_BITS);
            a.val[2] = T::ZERO;
            a.val[3] = T::ZERO;
            a.val[1] = b.val[1];
            a.val[0] = b.val[0];

            let mut r = [T::ZERO; 4];
            let mut buff = [T::ZERO; 2];

            let q = super::div_4s_by_2s(
                a.val[..4].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
                &mut buff,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());
            a.remove_leading_zeros();

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_divider_multiple<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for n in 0..100 {
            let n = BigUint::from(T::decomposition_from_u32(n as u32).to_vec());
            let b = gen_random_biguint::<T>(2 * T::NB_BITS);
            let mut a = &b * n;
            a.val.resize(4, T::ZERO);

            let mut r = [T::ZERO; 4];
            let mut buff = [T::ZERO; 2];

            let q = super::div_4s_by_2s(
                a.val[..4].try_into().unwrap(),
                b.val[..2].try_into().unwrap(),
                &mut r,
                &mut buff,
            );
            assert_eq!(r[2], T::ZERO);
            assert_eq!(r[3], T::ZERO);

            let q: BigUint<T> = q.into();
            let r = BigUint::from(r.to_vec());
            a.remove_leading_zeros();

            assert!(&r < &b);
            assert_eq!(a, q * b + r);
        }
    }

    #[test_with(u32, u64)]
    fn test_corner_case_3_by_2<T: Digit>() {
        let n = BigUint::from(vec![
            T::ZERO,
            T::ONE,
            T::MAX - T::ONE,
            T::ONE << (T::NB_BITS - 1),
        ]);
        let d = BigUint::from(vec![T::MAX, T::ONE << (T::NB_BITS - 1)]);

        println!("{:?}", n);
        println!("{:?}", d);
        let (q, r) = super::rem_div(&n, &d).unwrap();

        assert!(&r < &d);
        assert_eq!(((d * q) + r), n);
    }

    #[test_with(u32, u64)]
    fn test_zero_division<T: Digit>() {
        let d = BigUint::from(vec![T::MAX; 100]);
        let n = &d - T::ONE;

        let (q, r) = super::rem_div(&n, &d).unwrap();

        assert_eq!(r, n);
        assert_eq!(q, BigUint::default());
    }
}

#[cfg(test)]
#[cfg(feature = "rand")]
mod tests_full {
    use crate::gen_random_biguint;
    use crate::traits::Digit;
    use crate::Imported;
    use num_bigint::BigUint;
    use rand::distributions::Standard;
    use rand::prelude::Distribution;
    use rand::{thread_rng, Rng};
    use typed_test_gen::test_with;

    const SIZE0: usize = 100;
    const SIZE1: usize = 800;
    const SIZE2: usize = 1000;

    fn gen_n_random_values<T>(n: usize) -> Vec<T>
    where
        Standard: Distribution<T>,
    {
        let mut ret = Vec::<T>::with_capacity(n);
        for _ in 0..n {
            ret.push(thread_rng().gen::<T>());
        }
        ret
    }

    /// Randomize some tests to compare the result with num-bigint
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_random(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_random");

        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_b = gen_n_random_values::<u32>(size_2);

        let a = BigUint::new(vec_a.clone());
        let b = BigUint::new(vec_b.clone());
        let c = b / a;
        let should_get = c.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact");
        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_c = gen_n_random_values::<u32>(size_2 - size_1);

        let a = BigUint::new(vec_a.clone());
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact_p1(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact");
        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_c = gen_n_random_values::<u32>(size_2 - size_1);

        let a = BigUint::new(vec_a.clone()) + 1u32;
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone()) + 1;
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact_m1(n: usize, size_1: usize, size_2: usize) {
        assert!(size_1 < size_2);
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact");
        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_c = gen_n_random_values::<u32>(size_2 - size_1);

        let a = BigUint::new(vec_a.clone()) - 1u32;
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone()) - 1;
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_exact_scalar(n: usize, size: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact_scalar");
        let vec_a = gen_n_random_values::<u32>(size);
        let vec_c = gen_n_random_values::<u32>(1);

        let a = BigUint::new(vec_a.clone());
        let c = BigUint::new(vec_c.clone());
        let b = &a * &c;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_bi_scalar(n: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_exact_scalar");
        let vec_a = gen_n_random_values::<u32>(1);

        let a = BigUint::new(vec_a.clone());
        let mut b = BigUint::new(vec_a.clone());
        b += 1u32;
        let c = &b / &a;
        let should_get = c.to_u32_digits();
        let vec_b = b.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_same(n: usize, size: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_same");
        let vec_a = gen_n_random_values::<u32>(size);

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_a.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        let should_get = vec![1];
        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_based<T: Digit>(n: usize, size_1: usize, size_2: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_based");
        assert!(size_1 < size_2);
        let mut vec_a = vec![T::ZERO; size_1];
        let mut vec_b = vec![T::ZERO; size_2];
        *vec_a.last_mut().unwrap() = T::ONE;
        *vec_b.last_mut().unwrap() = T::ONE;

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        let mut should_get = vec![T::ZERO; size_2 - size_1 + 1];
        *should_get.last_mut().unwrap() = T::ONE;
        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn coherence_with_num_bigint_9<T: Digit>(n: usize, size: usize) {
        println!("STEP {n}");
        println!("coherence_with_num_bigint_9");
        let vec_a = vec![T::MAX];
        let vec_b = vec![T::MAX; size];

        let biguinta = crate::BigUint::from(vec_a.clone());
        let mut biguintb = crate::BigUint::from(vec_b.clone());
        // let got_biguint = &biguintb / &biguinta;
        // let got_main = got_biguint.val;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;

        let should_get = vec![T::ONE; size];
        assert_digits_equal(&should_get, &got_newt_raph);

        biguintb += T::ONE;
        let got_newt_raph = super::div(&biguintb, &biguinta).unwrap().val;
        assert_digits_equal(&should_get, &got_newt_raph);
    }

    fn assert_digits_equal<T: Digit>(lhs: &Vec<T>, rhs: &Vec<T>) {
        if lhs != rhs {
            // assert_eq!(got_schoolbook, lhs);
            assert_eq!(lhs.len(), rhs.len());

            let should_get_int = crate::BigInt::from(lhs.clone());
            let newt_raph_int = crate::BigInt::from(rhs.clone());
            let error = (should_get_int - newt_raph_int).to_string();
            println!("Error: {}", error.to_string());
            // assert!(error == "0" || error == "-1");

            for (i, (a, b)) in lhs.iter().zip(rhs.iter()).enumerate() {
                if a > b {
                    println!("digit {i}, diff {}", *a - *b);
                } else if b > a {
                    println!("digit {i}, diff {}", *b - *a);
                }
            }
        }

        assert_eq!(lhs, rhs);
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_many() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);

            coherence_with_num_bigint_random(n, size_0, size_2);
            coherence_with_num_bigint_random(n, size_0, size_1);
            coherence_with_num_bigint_random(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_0() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact(n, size_0, size_2);
            coherence_with_num_bigint_exact(n, size_0, size_1);
            coherence_with_num_bigint_exact(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_1() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact_p1(n, size_0, size_2);
            coherence_with_num_bigint_exact_p1(n, size_0, size_1);
            coherence_with_num_bigint_exact_p1(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_2() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact_m1(n, size_0, size_2);
            coherence_with_num_bigint_exact_m1(n, size_0, size_1);
            coherence_with_num_bigint_exact_m1(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_3() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_exact_scalar(n, size_0);
            coherence_with_num_bigint_exact_scalar(n, size_1);
            coherence_with_num_bigint_exact_scalar(n, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    fn coherence_with_num_bigint_4() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_same(n, size_0);
            coherence_with_num_bigint_same(n, size_1);
            coherence_with_num_bigint_same(n, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[ignore]
    fn coherence_with_num_bigint_7() {
        for n in 0..100 {
            coherence_with_num_bigint_bi_scalar(n);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test_with(u32, u64)]
    #[cfg(feature = "rand")]
    #[ignore]
    fn coherence_with_num_bigint_5<T: Digit>() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_based::<T>(n, size_0, size_2);
            coherence_with_num_bigint_based::<T>(n, size_0, size_1);
            coherence_with_num_bigint_based::<T>(n, size_1, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test_with(u32, u64)]
    #[cfg(feature = "rand")]
    #[ignore]
    fn coherence_with_num_bigint_6<T: Digit>() {
        for n in 0..100 {
            let size_0 = SIZE0 + rand::thread_rng().gen_range(0..100);
            let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
            let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);
            coherence_with_num_bigint_9::<T>(n, size_0);
            coherence_with_num_bigint_9::<T>(n, size_1);
            coherence_with_num_bigint_9::<T>(n, size_2);
        }
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test_with(u32, u64)]
    fn sqrt_2<T: Digit>() {
        let n = match Imported::<T>::read_from_file("src/export/test_files/sqrt_2_v1.tw").unwrap() {
            Imported::Uint(n) => n,
            _ => panic!(),
        };

        let ret = super::div(&n, &n).unwrap();
        assert_eq!(ret.to_string(), "1");

        let mut n2 = &n >> 1;
        let ret = super::div(&n, &n2).unwrap();
        assert_eq!(ret.to_string(), "2");

        n2 += T::ONE;
        let ret = super::div(&n, &n2).unwrap();
        assert_eq!(ret.to_string(), "1");
    }

    /// This triggers the special branch of the `div_3n_by_2n` function
    #[test_with(u32, u64)]
    #[ignore]
    fn test_corner_case_3_by_2<T: Digit>()
    where
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let d = gen_random_biguint::<T>(512 * T::NB_BITS);
            let n = (&d << (512 * T::NB_BITS)) - T::ONE;

            let (q, r) = super::rem_div(&n, &d).unwrap();
            assert!(&r < &d);
            assert_eq!(((d * q) + r), n);
        }
    }
}
