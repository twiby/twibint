use crate::BigUint;

impl BigUint {
    pub fn pow(&self, mut exp: usize) -> BigUint {
        if exp == 0 {
            return biguint!(1u32);
        }

        let mut base = self.clone();
        let mut ret = biguint!(1u32);

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
