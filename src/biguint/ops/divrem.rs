use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::errors::DivisionError;
use crate::traits::{Digit, DivisionResult, DoubleDigit, RemDiv};
use crate::BigUint;

use crate::biguint::ops::div;

impl<T: Digit> RemDiv<T> for BigUint<T> {
    type DivOutput = BigUint<T>;
    type RemOutput = T;
    fn rem_div(&self, other: &T) -> DivisionResult<(BigUint<T>, T)> {
        if *other == T::ZERO {
            return Err(DivisionError::DivisionByZero);
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

        ret.remove_leading_zeros();
        Ok((ret, msb.truncate_upper()))
    }

    fn rem(&self, other: &T) -> DivisionResult<T> {
        if *other == T::ZERO {
            return Err(DivisionError::DivisionByZero);
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
        *self = value;
    }
}
impl<T: Digit> RemAssign<&T> for BigUint<T> {
    fn rem_assign(&mut self, other: &T) {
        let value = &*self % *other;
        *self = value;
    }
}
impl<T: Digit> Rem<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: T) -> Self::Output {
        BigUint::new(RemDiv::rem(&self, &other).unwrap())
    }
}
impl<T: Digit> Rem<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: T) -> Self::Output {
        BigUint::new(RemDiv::rem(self, &other).unwrap())
    }
}
impl<T: Digit> Rem<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: &T) -> Self::Output {
        BigUint::new(RemDiv::rem(&self, other).unwrap())
    }
}
impl<T: Digit> Rem<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: &T) -> Self::Output {
        BigUint::new(RemDiv::rem(self, other).unwrap())
    }
}

impl<T: Digit> DivAssign<T> for BigUint<T> {
    fn div_assign(&mut self, other: T) {
        *self = &*self / other;
    }
}
impl<T: Digit> DivAssign<&T> for BigUint<T> {
    fn div_assign(&mut self, other: &T) {
        *self = &*self / *other;
    }
}
impl<T: Digit> Div<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: T) -> BigUint<T> {
        RemDiv::div(self, &other).unwrap()
    }
}
impl<T: Digit> Div<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: &T) -> BigUint<T> {
        RemDiv::div(self, other).unwrap()
    }
}
impl<T: Digit> Div<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: T) -> BigUint<T> {
        RemDiv::div(&self, &other).unwrap()
    }
}
impl<T: Digit> Div<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: &T) -> BigUint<T> {
        RemDiv::div(&self, other).unwrap()
    }
}

impl<T: Digit> RemDiv<BigUint<T>> for BigUint<T> {
    type DivOutput = BigUint<T>;
    type RemOutput = BigUint<T>;
    fn rem_div(&self, other: &BigUint<T>) -> DivisionResult<(BigUint<T>, BigUint<T>)> {
        div(self, other)
    }
}
impl<T: Digit> RemAssign<&BigUint<T>> for BigUint<T> {
    fn rem_assign(&mut self, other: &BigUint<T>) {
        *self = &*self % other;
    }
}
impl<T: Digit> RemAssign<BigUint<T>> for BigUint<T> {
    fn rem_assign(&mut self, other: BigUint<T>) {
        *self = &*self % &other;
    }
}
impl<T: Digit> Rem<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: &BigUint<T>) -> BigUint<T> {
        RemDiv::rem(self, other).unwrap()
    }
}
impl<T: Digit> Rem<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: &BigUint<T>) -> BigUint<T> {
        RemDiv::rem(&self, other).unwrap()
    }
}
impl<T: Digit> Rem<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: BigUint<T>) -> BigUint<T> {
        RemDiv::rem(&self, &other).unwrap()
    }
}
impl<T: Digit> Rem<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn rem(self, other: BigUint<T>) -> BigUint<T> {
        RemDiv::rem(self, &other).unwrap()
    }
}
impl<T: Digit> DivAssign<BigUint<T>> for BigUint<T> {
    fn div_assign(&mut self, other: BigUint<T>) {
        *self = &*self / &other;
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
impl<T: Digit> Div<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: BigUint<T>) -> BigUint<T> {
        RemDiv::div(&self, &other).unwrap()
    }
}
impl<T: Digit> Div<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: BigUint<T>) -> BigUint<T> {
        RemDiv::div(self, &other).unwrap()
    }
}
impl<T: Digit> Div<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn div(self, other: &BigUint<T>) -> BigUint<T> {
        RemDiv::div(&self, other).unwrap()
    }
}
