use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::traits::{Digit, DivisionResult, RemDiv};
use crate::{BigInt, BigUint};

impl<T: Digit> RemDiv<T> for BigInt<T> {
    type DivOutput = BigInt<T>;
    type RemOutput = T;
    fn rem_div(&self, other: &T) -> DivisionResult<(BigInt<T>, T)> {
        let (mut q, mut r) = self.uint.rem_div(other)?;

        if !self.sign {
            q += T::ONE;
            r = *other - r;
        }

        Ok((
            BigInt::<T> {
                uint: q,
                sign: self.sign,
            },
            r,
        ))
    }
    fn div(&self, other: &T) -> DivisionResult<BigInt<T>> {
        let mut q = RemDiv::div(&self.uint, other)?;

        if !self.sign {
            q += T::ONE;
        }

        Ok(BigInt::<T> {
            uint: q,
            sign: self.sign,
        })
    }
    fn rem(&self, other: &T) -> DivisionResult<T> {
        let mut r = RemDiv::rem(&self.uint, other)?;

        if !self.sign {
            r = *other - r;
        }

        Ok(r)
    }
}

impl<T: Digit> RemDiv<BigInt<T>> for BigInt<T> {
    type DivOutput = BigInt<T>;
    type RemOutput = BigInt<T>;
    fn rem_div(&self, other: &BigInt<T>) -> DivisionResult<(BigInt<T>, BigInt<T>)> {
        let (mut q, mut r) = self.uint.rem_div(&other.uint)?;

        if self.sign ^ other.sign {
            q += T::ONE;
            r = &other.uint - &r;
        }

        Ok((
            BigInt::<T> {
                uint: q,
                sign: !(self.sign ^ other.sign),
            },
            BigInt::<T> {
                uint: r,
                sign: other.sign,
            },
        ))
    }
    fn div(&self, other: &BigInt<T>) -> DivisionResult<BigInt<T>> {
        let mut q = RemDiv::<BigUint<T>>::div(&self.uint, &other.uint)?;

        if self.sign ^ other.sign {
            q += T::ONE;
        }

        Ok(BigInt::<T> {
            uint: q,
            sign: !(self.sign ^ other.sign),
        })
    }
    fn rem(&self, other: &BigInt<T>) -> DivisionResult<BigInt<T>> {
        let mut r = RemDiv::<BigUint<T>>::rem(&self.uint, &other.uint)?;

        if self.sign ^ other.sign {
            r = &other.uint - &r;
        }

        Ok(BigInt::<T> {
            uint: r,
            sign: other.sign,
        })
    }
}

impl<T: Digit> RemAssign<T> for BigInt<T> {
    fn rem_assign(&mut self, other: T) {
        let value = &*self % other;
        *self = BigInt::<T>::from(BigUint::<T>::new(value));
    }
}
impl<T: Digit> Rem<T> for &BigInt<T> {
    type Output = T;
    fn rem(self, other: T) -> T {
        RemDiv::rem(self, &other).unwrap()
    }
}
impl<T: Digit> Rem<T> for BigInt<T> {
    type Output = T;
    fn rem(self, other: T) -> T {
        RemDiv::rem(&self, &other).unwrap()
    }
}

impl<T: Digit> DivAssign<T> for BigInt<T> {
    fn div_assign(&mut self, other: T) {
        *self = &*self / other;
    }
}
impl<T: Digit> Div<T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn div(self, other: T) -> BigInt<T> {
        RemDiv::div(self, &other).unwrap()
    }
}
impl<T: Digit> Div<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn div(self, other: T) -> BigInt<T> {
        RemDiv::div(&self, &other).unwrap()
    }
}

impl<T: Digit> RemAssign<&BigInt<T>> for BigInt<T> {
    fn rem_assign(&mut self, other: &BigInt<T>) {
        *self = &*self % other;
    }
}
impl<T: Digit> Rem<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn rem(self, other: &BigInt<T>) -> BigInt<T> {
        RemDiv::rem(self, other).unwrap()
    }
}
impl<T: Digit> Rem<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn rem(self, other: BigInt<T>) -> BigInt<T> {
        RemDiv::rem(&self, &other).unwrap()
    }
}
impl<T: Digit> DivAssign<&BigInt<T>> for BigInt<T> {
    fn div_assign(&mut self, other: &BigInt<T>) {
        *self = &*self / other;
    }
}
impl<T: Digit> Div<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn div(self, other: &BigInt<T>) -> BigInt<T> {
        RemDiv::div(self, other).unwrap()
    }
}
impl<T: Digit> Div<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn div(self, other: BigInt<T>) -> BigInt<T> {
        RemDiv::div(&self, &other).unwrap()
    }
}
