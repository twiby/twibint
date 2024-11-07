use crate::biguint::ops::add_assign;
use crate::biguint::ops::sub_assign;
use crate::errors::DivisionError;
use crate::traits::Digit;
use crate::traits::DivisionResult;
use std::cmp::Ordering;

use crate::BigUint;

mod burnikel_ziegler;
mod multiplication_helper;
pub(crate) mod newton_raphson;

pub(crate) fn div<T: Digit>(
    n: &BigUint<T>,
    d: &BigUint<T>,
) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
    // Division by zero error
    if d == &BigUint::<T>::new(T::ZERO) {
        return Err(DivisionError::DivisionByZero);
    }

    // Early exit
    match n.cmp(d) {
        Ordering::Equal => return Ok((BigUint::<T>::new(T::ONE), BigUint::<T>::new(T::ZERO))),
        Ordering::Less => return Ok((BigUint::<T>::new(T::ZERO), n.clone())),
        _ => (),
    }

    // let mut q = vec![T::ZERO; n.val.len() - d.val.len() + 1];
    // let mut r = vec![T::ZERO; d.val.len()];
    // schoolbook_div(&n.val, &d.val, &mut q, &mut r)?;
    // return Ok((q.into(), r.into()));

    newton_raphson::rem_div(n, d)
}

/// q must be big enough for the result to fit
#[allow(unused)]
fn schoolbook_div<T: Digit>(n: &[T], d: &[T], q: &mut [T], r: &mut [T]) -> DivisionResult<()> {
    // Build returned objects
    debug_assert!(n.len() >= d.len());
    debug_assert!(r.len() == d.len());
    r.fill(T::ZERO);
    q.fill(T::ZERO);

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
                let mut product = BigUint::<T>::new(T::ZERO);

                // We add to the current product power of 2 by power of 2
                for bit in (0..T::NB_BITS).rev() {
                    let temp = BigUint::from(d.to_vec()) << bit;
                    if ord(&r, r_last_digit, &(&product + &temp).val) != Ordering::Less {
                        quotient |= T::ONE << bit;
                        product += temp;
                    }
                }

                sub_assign(r, &product.val[..product.val.len().min(d.len())]);
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
