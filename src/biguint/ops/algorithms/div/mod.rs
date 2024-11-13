use crate::biguint::ops::add_assign;
use crate::biguint::ops::sub_assign;
use crate::errors::DivisionError;
use crate::traits::Digit;
use crate::traits::DivisionResult;
use crate::traits::DoubleDigit;
use std::cmp::Ordering;

use crate::BigUint;

mod multiplication_helper;
pub(crate) mod newton_raphson;

pub(crate) fn div<T: Digit>(
    n: &BigUint<T>,
    d: &BigUint<T>,
) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
    match d.val.len() {
        0 => Err(DivisionError::DivisionByZero),
        1 => {
            if d.val[0] == T::ZERO {
                return Err(DivisionError::DivisionByZero);
            }
            let mut q = vec![T::ZERO; n.val.len()];
            let r = schoolbook_div_single_digit(&n.val, d.val[0], &mut q)?;
            Ok((q.into(), BigUint::new(r)))
        }
        _ => {
            // let mut q = vec![T::ZERO; n.val.len() - d.val.len() + 1];
            // let mut r = vec![T::ZERO; d.val.len()];
            // schoolbook_div(&n.val, &d.val, &mut q, &mut r)?;
            // Ok((q.into(), r.into()))
            newton_raphson::rem_div(n, d)
        }
    }
}

/// q must be big enough for the result to fit
#[allow(unused)]
fn schoolbook_div<T: Digit>(n: &[T], d: &[T], q: &mut [T], r: &mut [T]) -> DivisionResult<()> {
    // Build returned objects
    debug_assert!(n.len() >= d.len());
    debug_assert!(r.len() == d.len());
    r.fill(T::ZERO);
    q.fill(T::ZERO);

    let mut p = vec![T::ZERO; d.len() + 1]; // (to reduce ??)
    let mut shifted_d = BigUint::from(d.to_vec()) << T::NB_BITS;

    // Loop on the digits of n
    for (idx, digit) in n.iter().enumerate().rev() {
        let r_last_digit = r[d.len() - 1];
        r.copy_within(0..d.len() - 1, 1);
        r[0] = *digit;

        // Get the quotient remainder/other
        let quotient: T = match ord(&r, r_last_digit, d) {
            // remainder lower than other: accumulate one more digit of n
            Ordering::Less => continue,

            // remainder = other: quotient is 1
            Ordering::Equal => {
                r.fill(T::ZERO);
                T::ONE
            }

            // remainder greater than other: quotient is a positive u32
            Ordering::Greater => {
                let mut quotient = T::ZERO;
                p.fill(T::ZERO);

                // We add to the current product power of 2 by power of 2
                for bit in (0..T::NB_BITS).rev() {
                    shifted_d >>= 1;
                    add_assign(&mut p, &shifted_d.val);
                    match ord(&r, r_last_digit, &p) {
                        Ordering::Less => {
                            sub_assign(&mut p, &shifted_d.val);
                        }
                        _ => {
                            quotient |= T::ONE << bit;
                        }
                    };
                }

                shifted_d <<= T::NB_BITS;
                sub_assign(r, &p[..d.len()]);
                quotient
            }
        };

        // q[idx..] is guaranteed to be big enough to ignore carry
        let _ = add_assign(&mut q[idx..], &[quotient]);
    }

    Ok(())
}

/// Wrapper arund ord to deal with remainder that may have an additional MSB
fn ord<T: Digit>(r: &[T], r_last_digit: T, d: &[T]) -> Ordering {
    match d.len() - r.len() {
        0 => {
            if r_last_digit > T::ZERO {
                Ordering::Greater
            } else {
                crate::biguint::ord(r, d)
            }
        }
        1 => {
            let msb_ord = r_last_digit.cmp(&d[d.len() - 1]);
            if msb_ord != Ordering::Equal {
                msb_ord
            } else {
                crate::biguint::ord(r, &d[..d.len() - 1])
            }
        }
        _ => unreachable!(),
    }
}

fn schoolbook_div_single_digit<T: Digit>(n: &[T], d: T, q: &mut [T]) -> DivisionResult<T> {
    let mut r = T::Double::ZERO;

    // Loop on the digits of n
    for (idx, digit) in n.iter().enumerate().rev() {
        r <<= T::NB_BITS;
        r |= digit.to_double();

        let quotient = (r / d.to_double()).truncate_upper();
        r %= d.to_double();
        let _ = add_assign(&mut q[idx..], &[quotient]);
    }

    Ok(r.truncate_upper())
}
