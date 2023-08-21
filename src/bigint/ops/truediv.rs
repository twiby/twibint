use crate::errors::DivisionByZero;
use crate::traits::{Digit, TrueDiv};
use crate::BigInt;

#[cfg(target_endian = "little")]
impl<T: Digit> TrueDiv<BigInt<T>> for BigInt<T> {
    fn truediv(&self, n2: &BigInt<T>) -> Result<f64, DivisionByZero> {
        if !(self.sign ^ n2.sign) {
            Ok(self.uint.truediv(&n2.uint)?)
        } else {
            Ok(-self.uint.truediv(&n2.uint)?)
        }
    }
}
