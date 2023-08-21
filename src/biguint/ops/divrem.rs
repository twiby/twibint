use core::cmp::Ordering;
use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::errors::DivisionByZero;
use crate::traits::{Digit, DivisionResult, DoubleDigit, RemDiv};
use crate::BigUint;

use crate::biguint::ops::implem_choices::add_assign;

impl<T: Digit> RemDiv<T> for BigUint<T> {
    type DivOutput = BigUint<T>;
    type RemOutput = T;
    fn rem_div(&self, other: &T) -> DivisionResult<(BigUint<T>, T)> {
        if *other == T::ZERO {
            return Err(DivisionByZero());
        }

        let other_64 = other.to_double();
        let mut msb = T::Double::ZERO;
        let mut div: T::Double;

        let mut ret = BigUint::<T>::new(T::ZERO);
        for idx in (0..self.val.len()).rev() {
            let lsb = self.val[idx].to_double();

            let current = (msb << T::NB_BITS) | lsb;
            (div, msb) = (current / other_64, current % other_64);

            ret += BigUint::<T>::from(div.split()) << (T::NB_BITS * idx);
        }

        ret.remove_trailing_zeros();
        Ok((ret, msb.truncate_upper()))
    }

    fn rem(&self, other: &T) -> DivisionResult<T> {
        if *other == T::ZERO {
            return Err(DivisionByZero());
        }
        let other_64 = other.to_double();
        let mut msb = T::Double::ZERO;

        for val in self.val.iter().rev() {
            let current = (msb << T::NB_BITS) | val.to_double();
            msb = current % other_64;
        }

        Ok(msb.truncate_upper())
    }
}

impl<T: Digit> RemAssign<T> for BigUint<T> {
    fn rem_assign(&mut self, other: T) {
        let value = &*self % other;
        *self = BigUint::<T>::new(value);
    }
}
impl<T: Digit> Rem<T> for &BigUint<T> {
    type Output = T;
    fn rem(self, other: T) -> T {
        RemDiv::rem(self, &other).unwrap()
    }
}

impl<T: Digit> DivAssign<T> for BigUint<T> {
    fn div_assign(&mut self, other: T) {
        *self = &*self / other;
    }
}
impl<T: Digit> Div<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: T) -> BigUint<T> {
        RemDiv::div(self, &other).unwrap()
    }
}

impl<T: Digit> RemDiv<BigUint<T>> for BigUint<T> {
    type DivOutput = BigUint<T>;
    type RemOutput = BigUint<T>;
    fn rem_div(&self, other: &BigUint<T>) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
        // Division by zero error
        if other == &BigUint::<T>::new(T::ZERO) {
            return Err(DivisionByZero());
        }

        // Early exit
        match self.cmp(other) {
            Ordering::Equal => return Ok((BigUint::<T>::new(T::ONE), BigUint::<T>::new(T::ZERO))),
            Ordering::Less => return Ok((BigUint::<T>::new(T::ZERO), self.clone())),
            _ => (),
        }

        // Build returned objects
        let mut ret = BigUint::<T> {
            val: vec![T::ZERO; self.val.len()],
        };
        let mut remainder = BigUint::<T>::new(T::ZERO);

        // Loop on the digits of self
        for (idx, digit) in self.val.iter().enumerate().rev() {
            remainder.val.insert(0, *digit);
            remainder.remove_trailing_zeros();

            // Get the quotient remainder/other
            let quotient: T = match remainder.cmp(other) {
                // remainder lower than other: accumulate one more digit of self
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
                        let temp = other << bit;
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

        ret.remove_trailing_zeros();
        Ok((ret, remainder))
    }
}
impl<T: Digit> RemAssign<&BigUint<T>> for BigUint<T> {
    fn rem_assign(&mut self, other: &BigUint<T>) {
        *self = &*self % other;
    }
}
impl<T: Digit> Rem<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: &BigUint<T>) -> BigUint<T> {
        RemDiv::rem(self, other).unwrap()
    }
}
impl<T: Digit> DivAssign<&BigUint<T>> for BigUint<T> {
    fn div_assign(&mut self, other: &BigUint<T>) {
        *self = &*self / other;
    }
}
impl<T: Digit> Div<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: &BigUint<T>) -> BigUint<T> {
        RemDiv::div(self, other).unwrap()
    }
}
