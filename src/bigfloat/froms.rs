use crate::BigInt;
use crate::BigUint;

use crate::traits::Digit;

use crate::BigFloat;

fn clean_uint<T: Digit>(uint: &mut BigUint<T>) -> isize {
    let residual_scale = uint.val.iter().take_while(|d| d == &&T::ZERO).count();

    // As always, keep at least one zero
    let residual_scale = residual_scale.min(uint.val.len() - 1);

    uint.val.drain(..residual_scale);
    residual_scale.try_into().unwrap()
}

impl<T: Digit> From<BigInt<T>> for BigFloat<T> {
    fn from(mut val: BigInt<T>) -> Self {
        let scale = clean_uint(&mut val.uint);
        Self { int: val, scale }
    }
}

impl<T: Digit> From<BigUint<T>> for BigFloat<T> {
    fn from(mut val: BigUint<T>) -> Self {
        let scale = clean_uint(&mut val);
        Self {
            int: val.into(),
            scale,
        }
    }
}

impl<T: Digit> From<Vec<T>> for BigFloat<T> {
    fn from(val: Vec<T>) -> Self {
        Self::from(BigUint::from(val))
    }
}

impl<T: Digit> From<T> for BigFloat<T> {
    fn from(val: T) -> Self {
        Self::from(BigUint::new(val))
    }
}

impl<T: Digit> From<BigFloat<T>> for BigInt<T> {
    fn from(mut val: BigFloat<T>) -> BigInt<T> {
        if val.scale < 0 {
            let cutoff = -val.scale as usize;

            if val.int.uint.val.len() <= cutoff {
                return Default::default();
            }

            val.int.uint.val.drain(..cutoff);
            let mut ret = BigInt {
                sign: val.int.sign,
                uint: BigUint::from(val.int.uint.val),
            };
            if !val.int.sign && ret > BigInt::default() {
                ret -= T::ONE;
            }
            ret
        } else {
            let scale = val.scale as usize;
            let mut int = val.int;
            int.uint <<= scale * T::NB_BITS;
            int
        }
    }
}
