use crate::biguint::ops::truediv::TrueDiv;
use crate::errors::DivisionByZero;
use crate::BigInt;

#[cfg(target_endian = "little")]
impl TrueDiv<BigInt> for BigInt {
    fn truediv(&self, n2: &BigInt) -> Result<f64, DivisionByZero> {
        if !(self.sign ^ n2.sign) {
            Ok(self.uint.truediv(&n2.uint)?)
        } else {
            Ok(-self.uint.truediv(&n2.uint)?)
        }
    }
}
