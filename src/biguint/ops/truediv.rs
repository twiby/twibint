use crate::biguint::ops::divrem::RemDiv;
use crate::BigUint;

pub trait TrueDiv<T> {
    fn truediv(&self, other: &T) -> f64;
}

#[cfg(target_endian = "little")]
impl TrueDiv<BigUint> for BigUint {
    fn truediv(&self, n2: &BigUint) -> f64 {
        // Compute exponent, biased by 52
        let mut exponent: i64 = 53i64 - (self.nb_bits() as i64) + (n2.nb_bits() as i64);

        // Actual division
        let n1 = match exponent {
            i if i > 0 => self << i.try_into().unwrap(),
            i if i < 0 => self >> (-i).try_into().unwrap(),
            _ => self.clone(),
        };
        let (mut q, r) = n1.rem_div(n2).unwrap();

        // rounding
        if r > (n2 >> 1) {
            q += 1;
        }

        // Get the correct 52 bits of mantissa
        let extra_bits = q.nb_bits() - 53;
        q >>= extra_bits;
        exponent -= extra_bits as i64;
        q.val[1] ^= 1u32 << 20;

        // Handle overflow or underflow (probably not correct, some answers get mapped to NaNs)
        if exponent > 52 + 1023 {
            return 0f64;
        } else if exponent <= -4096i64 + 52 + 1023 {
            return f64::INFINITY;
        }

        // Get actual mantissa and exponent biased 1023
        let mantissa: u64 = q.try_into().unwrap();
        let exponent_u64: u64 = (52 + 1023 - exponent).try_into().unwrap();
        f64::from_bits((exponent_u64 << 52) | mantissa)
    }
}
