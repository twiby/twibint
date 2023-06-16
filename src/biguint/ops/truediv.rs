use crate::BigUint;

pub trait TrueDiv<T> {
    fn truediv(&self, other: &T);
}

impl TrueDiv<BigUint> for BigUint {
    fn truediv(&self, n2: &BigUint) {
        assert!(self < n2);
        let n1 = self << 52 - self.nb_bits() + n2.nb_bits();

        let q = &n1 / n2;
        assert_eq!(q.nb_bits(), 52);
        assert_eq!(q.val.len(), 2);
    }
}
