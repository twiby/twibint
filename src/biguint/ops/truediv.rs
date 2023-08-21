use crate::errors::DivisionByZero;
use crate::traits::{Digit, RemDiv, TrueDiv};
use crate::BigUint;

#[cfg(target_endian = "little")]
impl<T: Digit> TrueDiv<BigUint<T>> for BigUint<T> {
    fn truediv(&self, n2: &BigUint<T>) -> Result<f64, DivisionByZero> {
        // Compute exponent, biased by 52
        let mut exponent: i64 = 53i64 - (self.nb_bits() as i64) + (n2.nb_bits() as i64);

        // Actual division
        let n1 = match exponent {
            i if i > 0 => self << i.try_into().unwrap(),
            i if i < 0 => self >> (-i).try_into().unwrap(),
            _ => self.clone(),
        };
        let (mut q, r) = n1.rem_div(n2)?;

        // rounding
        if r > (n2 >> 1) {
            q += T::ONE;
        }

        // Get the correct 52 bits of mantissa
        let extra_bits = q.nb_bits() - 53;
        q >>= extra_bits;
        exponent -= extra_bits as i64;

        // Handle overflow or underflow (probably not correct, some answers get mapped to NaNs)
        if exponent > 52 + 1023 {
            return Ok(0f64);
        } else if exponent <= -4096i64 + 52 + 1023 {
            return Ok(f64::INFINITY);
        }

        // Get actual mantissa and exponent biased 1023
        let mut mantissa: u64 = q.into();
        mantissa ^= 1u64 << 52;
        let exponent_u64: u64 = (52 + 1023 - exponent).try_into().unwrap();
        Ok(f64::from_bits((exponent_u64 << 52) | mantissa))
    }
}
