use crate::BigInt;

use crate::traits::Digit;

use crate::BigFloat;

impl<T: Digit> std::fmt::Binary for BigFloat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scale == 0 {
            <BigInt<T> as std::fmt::Binary>::fmt(&self.int, f)
        } else if self.scale > 0 {
            <BigInt<T> as std::fmt::Binary>::fmt(&self.int, f)?;
            let mut word = "".to_string();
            for _ in 0..T::NB_BITS {
                word.push('0');
            }
            for _ in 0..self.scale {
                write!(f, "{}", word)?;
            }
            Ok(())
        } else {
            let scale = (-self.scale) as usize;
            if self.int.uint.val.len() > scale {
                crate::bigint::fmt::bin(self.int.sign, &self.int.uint.val[scale..], f)?;
            } else {
                crate::bigint::fmt::bin::<T>(self.int.sign, &[], f)?;
            }
            write!(f, ".")?;
            crate::biguint::fmt::bin(&self.int.uint.val[..scale], f)
        }
    }
}
