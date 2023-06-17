use crate::BigUint;

pub trait TrueDiv<T> {
    fn truediv(&self, other: &T) -> f64;
}

impl TrueDiv<BigUint> for BigUint {
    fn truediv(&self, n2: &BigUint) -> f64 {
        assert!(self < n2);
        let mut exponent: u64 = (53 - self.nb_bits() + n2.nb_bits()).try_into().unwrap();
        let n1 = self << exponent.try_into().unwrap();

        let mut q = &n1 / n2;
        assert_eq!(q.nb_bits(), 53);
        q -= BigUint::from(1u32) << 52;
        assert_eq!(q.nb_bits(), 52);
        assert_eq!(q.val.len(), 2);

        let mantissa: u64 = (q.val[0] as u64) + ((q.val[1] as u64) << 32);

        exponent = 52 + 1023 - exponent;
        assert!(exponent < (1u64 << 12));
        assert!(mantissa < (1u64 << 53));

        f64::from_bits((exponent << 52) | mantissa)
    }
}
