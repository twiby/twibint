use crate::biguint::ops::mul;
use crate::biguint::ops::rsub_assign;
use crate::biguint::ops::sub_assign;
use crate::biguint::ord;
use crate::traits::Digit;
use crate::traits::DivisionResult;
use crate::traits::DoubleDigit;
use crate::BigFloat;
use crate::BigInt;
use std::cmp::Ordering;

use crate::BigUint;

#[derive(Clone, Copy, Debug)]
enum Quotient3By2<T: Digit> {
    Single(T),
    MaxPlusOne,
    MaxPlusTwo,
}

#[derive(Clone, Copy, Debug)]
enum Quotient4By2<T: Digit> {
    Single([T; 2]),
    MaxPlusOnePlus([T; 2]),
}

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

impl<T: Digit> Into<BigUint<T>> for Quotient3By2<T> {
    fn into(self) -> BigUint<T> {
        match self {
            Self::Single(n) => BigUint::new(n),
            Self::MaxPlusOne => BigUint::new(T::MAX) + T::ONE,
            Self::MaxPlusTwo => BigUint::new(T::MAX) + T::ONE + T::ONE,
        }
    }
}

impl<T: Digit> Into<BigUint<T>> for Quotient4By2<T> {
    fn into(self) -> BigUint<T> {
        match self {
            Self::Single(arr) => BigUint::from(arr.to_vec()),
            Self::MaxPlusOnePlus(arr) => BigUint::from([arr[0], arr[1], T::ONE].to_vec()),
        }
    }
}

/// A lot of assumtions here
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

fn div_3n_by_2n<T: Digit>(n: &[T], d: &[T], q: &mut [T], r: &mut [T], count: usize) {
    let size = d.len();
    assert_eq!(size % 2, 0);
    assert_eq!(n.len(), 3 * (size / 2));
    assert_eq!(q.len(), size / 2);
    assert_eq!(r.len(), size);
    assert!(d[size - 1] > (T::MAX >> 1));

    let mut big_r = match ord(&n[size..], &d[size / 2..]) {
        Ordering::Less => {
            div_4n_by_2n(
                &n[size / 2..],
                &d[size / 2..],
                q,
                &mut r[size / 2..],
                count + 1,
            );
            r[..size / 2].fill(T::ZERO);
            BigInt::from(r.to_vec())
        }
        _ => {
            panic!();
            q.fill(T::MAX);
            let mut big_r = BigFloat::from(n[size / 2..].to_vec());
            big_r.add_assign(true, 0, &d[size / 2..]);
            big_r.sub_assign(true, (size / 2) as isize, &d[size / 2..]);
            assert!(big_r.scale >= 0);
            big_r.int << (big_r.scale as usize + size / 2)
        }
    };

    let mut x = vec![T::ZERO; size];
    mul(&mut x, q, &d[..size / 2]);
    big_r.add_assign(true, &n[..size / 2]);
    let x_len = x.len() - x.iter().rev().take_while(|&&n| n == T::ZERO).count();
    big_r.sub_assign(true, &x[..x_len]);

    while big_r.is_sign_negative() {
        big_r.add_assign(true, d);
        sub_assign(q, &[T::ONE]);
    }

    assert!(big_r.uint.val.len() <= r.len());
    for i in 0..r.len() {
        if i < big_r.uint.val.len() {
            r[i] = big_r.uint.val[i];
        } else {
            r[i] = T::ZERO;
        }
    }
}

const RECURISION_THRESHOLD: usize = 2;
fn div_4n_by_2n<T: Digit>(n: &[T], d: &[T], q: &mut [T], r: &mut [T], count: usize) {
    let size = d.len();
    assert_eq!(n.len(), size * 2);
    assert_eq!(q.len(), size);
    assert_eq!(r.len(), size);
    assert!(d[size - 1] > (T::MAX >> 1));

    if size <= RECURISION_THRESHOLD || size % 2 != 0 {
        // Handle odd case ?

        // Normal division
        super::div_buffed(n, d, q, r).unwrap();
        return;
    }
    assert_eq!(size % 2, 0);

    div_3n_by_2n(&n[size / 2..], d, &mut q[size / 2..], r, count);

    let mut new_n = r.to_vec();
    new_n.resize(3 * (size / 2), T::ZERO);
    new_n.copy_within(..size, size / 2);
    new_n[..size / 2].copy_from_slice(&n[..size / 2]);

    div_3n_by_2n(&new_n, d, &mut q[..size / 2], r, count);
}

fn rem_div<T: Digit>(n: &BigUint<T>, d: &BigUint<T>) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
    let d_bits = d.nb_bits();
    let size = d.val.len().next_power_of_two();
    let size_bits = size * T::NB_BITS;
    let shift = size_bits - d_bits;
    let mut n_data = (n << shift).val;
    let d_data = (d << shift).val;

    if n_data.len() > d_data.len() * 2 {
        return super::div(n, d);
    }

    assert_eq!(d_data.len(), size);
    assert!(n_data.len() <= d_data.len() * 2);
    n_data.resize(d_data.len() * 2, T::ZERO);
    let mut q = vec![T::ZERO; size];
    let mut r = vec![T::ZERO; size];

    div_4n_by_2n(&n_data, &d_data, &mut q, &mut r, 0);

    let r = BigUint::from(r) >> shift;
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
}

#[cfg(test)]
#[cfg(feature = "rand")]
mod tests_full {
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
}
