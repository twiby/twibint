//! (private) froms: private module containing all `From<T>`implementations
//!
//! These implementations are meant to be the main to construct a BigUint,
//! or export its value into another type.

use crate::biguint::Digits;
use crate::errors::FromFloatError;
use crate::errors::UnexpectedCharacterError;
use crate::BigUint;

impl From<&BigUint> for f64 {
    fn from(int: &BigUint) -> f64 {
        let mut exponent = 0i64;

        // Get the correct 52 bits of mantissa
        let extra_bits = int.nb_bits() - 53;
        let mut cleaned = (int >> extra_bits) + (int.bit(extra_bits - 1) as u32);

        exponent -= extra_bits as i64;
        cleaned.val[1] ^= 1u32 << 20;

        // Handle overflow or underflow (probably not correct, some answers get mapped to NaNs)
        if exponent > 52 + 1023 {
            return 0f64;
        } else if exponent <= -4096i64 + 52 + 1023 {
            return f64::INFINITY;
        }

        // Get actual mantissa and exponent biased 1023
        let mantissa: u64 = cleaned.try_into().unwrap();
        let exponent_u64: u64 = (52 + 1023 - exponent).try_into().unwrap();
        f64::from_bits((exponent_u64 << 52) | mantissa)
    }
}

impl From<&BigUint> for f32 {
    fn from(int: &BigUint) -> f32 {
        let mut exponent = 0i32;

        // Get the correct 23 bits of mantissa
        let extra_bits = int.nb_bits() - 24;
        let mut cleaned = (int >> extra_bits) + (int.bit(extra_bits - 1) as u32);

        exponent -= extra_bits as i32;
        cleaned.val[0] ^= 1u32 << 23;

        // Handle overflow or underflow (probably not correct, some answers get mapped to NaNs)
        if exponent > 23 + 127 {
            return 0f32;
        } else if exponent <= -512 + 23 + 127 {
            return f32::INFINITY;
        }

        // Get actual mantissa and exponent biased 127
        let mantissa: u32 = cleaned.try_into().unwrap();
        let exponent_u32: u32 = (23 + 127 - exponent).try_into().unwrap();
        f32::from_bits((exponent_u32 << 23) | mantissa)
    }
}

impl std::str::FromStr for BigUint {
    type Err = UnexpectedCharacterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = BigUint::new(0);

        let mut base = BigUint::new(1);
        for c in s.chars().rev() {
            let v: u32 = match c.to_digit(10) {
                Some(val) => val,
                None => return Err(UnexpectedCharacterError(c)),
            };

            ret += v * &base;
            base *= 10u32;
        }

        ret.remove_trailing_zeros();
        return Ok(ret);
    }
}

impl From<&BigUint> for Digits {
    fn from(b: &BigUint) -> Digits {
        let mut digits = Digits::new(0);

        for bit in b.bits().rev() {
            digits.times_2();
            if bit {
                digits.add_n_at_k(1, 0);
            }
        }

        digits
    }
}

impl From<&BigUint> for String {
    fn from(b: &BigUint) -> String {
        String::from(&Digits::from(b))
    }
}

impl From<BigUint> for String {
    fn from(b: BigUint) -> String {
        String::from(&Digits::from(&b))
    }
}

impl From<Vec<u32>> for BigUint {
    fn from(v: Vec<u32>) -> BigUint {
        let mut ret = BigUint { val: v };
        ret.remove_trailing_zeros();
        ret
    }
}

impl From<&str> for BigUint {
    fn from(s: &str) -> BigUint {
        s.parse().unwrap()
    }
}
impl From<String> for BigUint {
    fn from(s: String) -> BigUint {
        BigUint::from(s.as_str())
    }
}

impl From<u64> for BigUint {
    fn from(n: u64) -> BigUint {
        let mut ret = BigUint {
            val: vec![n as u32, (n >> 32) as u32],
        };
        ret.remove_trailing_zeros();
        ret
    }
}
impl From<u32> for BigUint {
    fn from(n: u32) -> BigUint {
        BigUint::new(n)
    }
}

#[derive(Debug)]
pub struct IntoUintError {}
impl TryFrom<&BigUint> for u64 {
    type Error = IntoUintError;
    fn try_from(uint: &BigUint) -> Result<u64, IntoUintError> {
        match uint.val.len() {
            0 => unreachable!(),
            1 => Ok(uint.val[0].into()),
            2 => Ok(uint.val[0] as u64 + ((uint.val[1] as u64) << 32)),
            _ => Err(IntoUintError {}),
        }
    }
}
impl TryFrom<BigUint> for u64 {
    type Error = IntoUintError;
    fn try_from(uint: BigUint) -> Result<u64, IntoUintError> {
        u64::try_from(&uint)
    }
}
impl TryFrom<&BigUint> for u32 {
    type Error = IntoUintError;
    fn try_from(uint: &BigUint) -> Result<u32, IntoUintError> {
        match uint.val.len() {
            0 => unreachable!(),
            1 => Ok(uint.val[0]),
            _ => Err(IntoUintError {}),
        }
    }
}
impl TryFrom<BigUint> for u32 {
    type Error = IntoUintError;
    fn try_from(uint: BigUint) -> Result<u32, IntoUintError> {
        u32::try_from(&uint)
    }
}

#[cfg(target_endian = "little")]
impl TryFrom<f64> for BigUint {
    type Error = FromFloatError<f64>;

    fn try_from(f: f64) -> Result<BigUint, FromFloatError<f64>> {
        if f != 0f64 && !f.is_normal() {
            return Err(FromFloatError::NotNormal(f));
        } else if f.is_sign_negative() {
            return Err(FromFloatError::Negative(f));
        }

        let f_u64: u64 = f.to_bits();

        let two_to_the_52 = 1 << 52;
        let mantissa_mask = two_to_the_52 - 1;

        let exponent = ((f_u64 >> 52) as i64) - 1023 - 52;
        let mantissa = two_to_the_52 | (f_u64 & mantissa_mask);

        Ok(match exponent {
            i if i < 0 => BigUint::from(mantissa) >> exponent.abs().try_into().unwrap(),
            i if i > 0 => BigUint::from(mantissa) << exponent.try_into().unwrap(),
            _ => BigUint::from(mantissa),
        })
    }
}

#[cfg(target_endian = "little")]
impl TryFrom<f32> for BigUint {
    type Error = FromFloatError<f32>;

    fn try_from(f: f32) -> Result<BigUint, FromFloatError<f32>> {
        if f != 0f32 && !f.is_normal() {
            return Err(FromFloatError::NotNormal(f));
        } else if f.is_sign_negative() {
            return Err(FromFloatError::Negative(f));
        }

        let f_u32: u32 = f.to_bits();

        let two_to_the_23 = 1 << 23;
        let mantissa_mask = two_to_the_23 - 1;

        let exponent = ((f_u32 >> 23) as i64) - 127 - 23;
        let mantissa = two_to_the_23 | (f_u32 & mantissa_mask);

        Ok(match exponent {
            i if i < 0 => BigUint::from(mantissa) >> exponent.abs().try_into().unwrap(),
            i if i > 0 => BigUint::from(mantissa) << exponent.try_into().unwrap(),
            _ => BigUint::from(mantissa),
        })
    }
}
