use crate::traits::{Digit, Pow};
use crate::BigUint;

impl<T: Digit> Pow for BigUint<T> {
    fn pow(&self, mut exp: usize) -> BigUint<T> {
        if exp == 0 {
            return BigUint::<T>::new(T::ONE);
        }

        let mut base = self.clone();
        let mut ret = BigUint::<T>::new(T::ONE);

        while exp > 1 {
            if exp & 1 != 0 {
                ret *= &base;
            }
            base = &base * &base;
            exp >>= 1;
        }

        ret * base
    }
}
