use crate::biguint::ops::add_assign;
use crate::errors::DivisionError;
use crate::traits::Digit;
use crate::traits::DivisionResult;
use std::cmp::Ordering;

use crate::BigUint;

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

    newton_raphson::rem_div(n, d)
}

#[allow(unused)]
fn schoolbook_div<T: Digit>(
    n: &BigUint<T>,
    d: &BigUint<T>,
) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
    // Build returned objects
    let mut ret = BigUint::<T> {
        val: vec![T::ZERO; n.val.len()],
    };
    let mut remainder = BigUint::<T>::new(T::ZERO);

    // Loop on the digits of n
    for (idx, digit) in n.val.iter().enumerate().rev() {
        remainder.val.insert(0, *digit);
        remainder.remove_leading_zeros();

        // Get the quotient remainder/other
        let quotient: T = match remainder.cmp(d) {
            // remainder lower than other: accumulate one more digit of n
            Ordering::Less => continue,

            // remainder = other: quotient is 1
            Ordering::Equal => {
                remainder.val.resize(1, T::ZERO);
                remainder.val[0] = T::ZERO;
                T::ONE
            }

            // remainder greater than other: quotient is a positive u32
            Ordering::Greater => {
                let mut quotient = T::ZERO;
                let mut product = BigUint::<T>::new(T::ZERO);

                // We add to the current product power of 2 by power of 2
                for bit in (0..T::NB_BITS).rev() {
                    let temp = d << bit;
                    if &product + &temp <= remainder {
                        quotient |= T::ONE << bit;
                        product += temp;
                    }
                }

                remainder -= &product;
                quotient
            }
        };

        // ret.val[idx..] is guaranteed to be big enough to ignore carry
        let _ = add_assign(&mut ret.val[idx..], &[quotient]);
    }

    ret.remove_leading_zeros();
    Ok((ret, remainder))
}
