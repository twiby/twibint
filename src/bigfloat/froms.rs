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
