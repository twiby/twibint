use crate::biguint::ops::truediv::TrueDiv;
use crate::BigInt;

#[cfg(target_endian = "little")]
impl TrueDiv<BigInt> for BigInt {
    fn truediv(&self, n2: &BigInt) -> f64 {
        if !(self.sign ^ n2.sign) {
            self.uint.truediv(&n2.uint)
        } else {
            -self.uint.truediv(&n2.uint)
        }
    }
}
