use crate::BigUint;

pub trait TrueDiv<T> {
    fn truediv(&self, other: &T) -> f64;
}

#[cfg(target_endian = "little")]
impl TrueDiv<BigUint> for BigUint {
    fn truediv(&self, n2: &BigUint) -> f64 {
        assert!(self < n2);

        // Compute exponent, biased by 52
        let mut exponent: u64 = (53 - self.nb_bits() + n2.nb_bits()).try_into().unwrap();
        if exponent > 52 + 1023 {
            return f64::INFINITY;
        }

        // Actual division
        let n1 = self << exponent.try_into().unwrap();
        let mut q = &n1 / n2;

        // Get the correct 52 bits of mantissa
        let extra_bits = q.nb_bits() - 53;
        q >>= extra_bits;
        assert_eq!(q.nb_bits(), 53);
        q -= BigUint::from(1u32) << 52;
        assert!(q.nb_bits() < 53);

        // Get actual mantissa and exponent biased 1023
        let mantissa: u64 = q.try_into().unwrap();
        exponent = 52 + 1023 - exponent;

        assert!(exponent < (1u64 << 12));
        assert!(mantissa < (1u64 << 53));

        f64::from_bits((exponent << 52) | mantissa)
    }
}
