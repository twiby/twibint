use crate::biguint::ops::truediv::TrueDiv;
use crate::BigInt;

#[cfg(target_endian = "little")]
impl TrueDiv<BigInt> for BigInt {
    fn truediv(&self, n2: &BigInt) -> Option<f64> {
        if !(self.sign ^ n2.sign) {
            Some(self.uint.truediv(&n2.uint)?)
        } else {
            Some(-self.uint.truediv(&n2.uint)?)
        }
    }
}
