//! (private) froms: private module containing all `From<T>`implementations
//!
//! These implementations are meant to be the main to construct a BigUint,
//! or export its value into another type.

use core::cmp::Ordering;

use crate::biguint::Digits;
use crate::errors::FromFloatError;
use crate::errors::UnexpectedCharacterError;
use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> From<&BigUint<T>> for f64 {
    fn from(int: &BigUint<T>) -> f64 {
        let mut exponent = 0i64;

        // Get the correct 52 bits of mantissa
        let extra_bits = int.nb_bits() - 53;
        let cleaned = (int >> extra_bits) + T::from(int.bit(extra_bits - 1));

        exponent -= extra_bits as i64;

        // Handle overflow or underflow (probably not correct, some answers get mapped to NaNs)
        if exponent > 52 + 1023 {
            return 0f64;
        } else if exponent <= -4096i64 + 52 + 1023 {
            return f64::INFINITY;
        }

        // Get actual mantissa and exponent biased 1023
        let mut mantissa: u64 = cleaned.into();
        mantissa ^= 1u64 << 52;
        let exponent_u64: u64 = (52 + 1023 - exponent).try_into().unwrap();
        f64::from_bits((exponent_u64 << 52) | mantissa)
    }
}

impl<T: Digit> From<&BigUint<T>> for f32 {
    fn from(int: &BigUint<T>) -> f32 {
        let mut exponent = 0i32;

        // Get the correct 23 bits of mantissa
        let extra_bits = int.nb_bits() - 24;
        let cleaned = (int >> extra_bits) + T::from(int.bit(extra_bits - 1));

        exponent -= extra_bits as i32;

        // Handle overflow or underflow (probably not correct, some answers get mapped to NaNs)
        if exponent > 23 + 127 {
            return 0f32;
        } else if exponent <= -512 + 23 + 127 {
            return f32::INFINITY;
        }

        // Get actual mantissa and exponent biased 127
        let mut mantissa: u32 = cleaned.into();
        mantissa ^= 1u32 << 23;
        let exponent_u32: u32 = (23 + 127 - exponent).try_into().unwrap();
        f32::from_bits((exponent_u32 << 23) | mantissa)
    }
}

impl<T: Digit> std::str::FromStr for BigUint<T> {
    type Err = UnexpectedCharacterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = BigUint::<T>::new(T::ZERO);

        let mut base = BigUint::<T>::new(T::ONE);
        for c in s.chars().rev() {
            let v: BigUint<T> = match c.to_digit(10) {
                Some(val) => val,
                None => return Err(UnexpectedCharacterError(c)),
            }
            .into();

            ret += &base * &v;
            base *= BigUint::<T>::from(10u32);
        }

        ret.remove_trailing_zeros();
        return Ok(ret);
    }
}

impl<T: Digit> From<&BigUint<T>> for Digits {
    fn from(b: &BigUint<T>) -> Digits {
        let mut digits = Digits::new(0);

        for bit in b.bits().rev() {
            digits.times_2();
            digits.add_n_at_k(bit as u8, 0);
        }

        digits
    }
}

impl<T: Digit> From<&BigUint<T>> for String {
    fn from(b: &BigUint<T>) -> String {
        String::from(&Digits::from(b))
    }
}

impl<T: Digit> From<BigUint<T>> for String {
    fn from(b: BigUint<T>) -> String {
        String::from(&Digits::from(&b))
    }
}

impl<T: Digit> From<Vec<T>> for BigUint<T> {
    fn from(v: Vec<T>) -> BigUint<T> {
        let mut ret = BigUint::<T> { val: v };
        ret.remove_trailing_zeros();
        ret
    }
}

impl<T: Digit> From<&str> for BigUint<T> {
    fn from(s: &str) -> BigUint<T> {
        s.parse().unwrap()
    }
}
impl<T: Digit> From<String> for BigUint<T> {
    fn from(s: String) -> BigUint<T> {
        BigUint::<T>::from(s.as_str())
    }
}

impl<T: Digit> From<(T, T)> for BigUint<T> {
    fn from((a, b): (T, T)) -> BigUint<T> {
        BigUint::<T>::from(vec![a, b])
    }
}

#[derive(Debug)]
pub enum IntoUintError {
    ToUint16Overflow,
    ToUint32Overflow,
    ToUint64Overflow,
    ToDigitOverflow,
    ToDoubleDigitOverflow,
}

impl<T: Digit> BigUint<T> {
    pub fn try_into_double_digit(&self) -> Result<T::Double, IntoUintError> {
        match self.val.len() {
            0 => unreachable!(),
            1 => Ok(self.val[0].to_double()),
            2 => Ok(self.val[0].to_double() + (self.val[1].to_double() << T::NB_BITS)),
            _ => Err(IntoUintError::ToDoubleDigitOverflow),
        }
    }

    pub fn try_into_digit(&self) -> Result<T, IntoUintError> {
        match self.val.len() {
            0 => unreachable!(),
            1 => Ok(self.val[0]),
            _ => Err(IntoUintError::ToDigitOverflow),
        }
    }
}

impl<T: Digit> TryFrom<&BigUint<T>> for u16 {
    type Error = IntoUintError;
    fn try_from(uint: &BigUint<T>) -> Result<u16, Self::Error> {
        let val_64: u64 = uint.try_into()?;
        match val_64.try_into() {
            Err(_) => Err(IntoUintError::ToUint16Overflow),
            Ok(val) => Ok(val),
        }
    }
}
impl<T: Digit> TryFrom<&BigUint<T>> for u32 {
    type Error = IntoUintError;
    fn try_from(uint: &BigUint<T>) -> Result<u32, Self::Error> {
        let val_64: u64 = uint.try_into()?;
        match val_64.try_into() {
            Err(_) => Err(IntoUintError::ToUint32Overflow),
            Ok(val) => Ok(val),
        }
    }
}
impl<T: Digit> TryFrom<&BigUint<T>> for u64 {
    type Error = IntoUintError;
    fn try_from(uint: &BigUint<T>) -> Result<u64, Self::Error> {
        match T::NB_BITS.cmp(&64) {
            Ordering::Greater => unreachable!(),
            Ordering::Equal | Ordering::Less => {
                assert_eq!(64 % T::NB_BITS, 0);

                let mut idx = 0;
                let mut ret = 0u64;
                let mut remaining = 64;
                while remaining >= T::NB_BITS && idx < uint.val.len() {
                    let val: u64 = match uint.val[idx].try_into() {
                        Err(_) => return Err(IntoUintError::ToUint64Overflow),
                        Ok(val) => val,
                    };
                    ret |= val << (idx * T::NB_BITS);
                    idx += 1;
                    remaining -= T::NB_BITS;
                }
                if idx < uint.val.len() {
                    return Err(IntoUintError::ToUint64Overflow);
                }
                Ok(ret)
            }
        }
    }
}

impl<T: Digit> From<u32> for BigUint<T> {
    fn from(n: u32) -> BigUint<T> {
        BigUint::<T>::from(T::decomposition_from_u32(n))
    }
}

impl<T: Digit> From<u64> for BigUint<T> {
    fn from(n: u64) -> BigUint<T> {
        BigUint::<T>::from(T::decomposition_from_u64(n))
    }
}

impl<T: Digit> From<BigUint<T>> for u16 {
    fn from(uint: BigUint<T>) -> u16 {
        <u16 as TryFrom<&BigUint<T>>>::try_from(&uint).unwrap()
    }
}
impl<T: Digit> From<BigUint<T>> for u32 {
    fn from(uint: BigUint<T>) -> u32 {
        <u32 as TryFrom<&BigUint<T>>>::try_from(&uint).unwrap()
    }
}
impl<T: Digit> From<BigUint<T>> for u64 {
    fn from(uint: BigUint<T>) -> u64 {
        <u64 as TryFrom<&BigUint<T>>>::try_from(&uint).unwrap()
    }
}

#[cfg(target_endian = "little")]
impl<T: Digit> TryFrom<f64> for BigUint<T> {
    type Error = FromFloatError<f64>;

    fn try_from(f: f64) -> Result<BigUint<T>, FromFloatError<f64>> {
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
        let ret = BigUint::<T>::from(T::decomposition_from_u64(mantissa));

        Ok(match exponent {
            i if i < 0 => ret >> exponent.abs().try_into().unwrap(),
            i if i > 0 => ret << exponent.try_into().unwrap(),
            _ => ret,
        })
    }
}

#[cfg(target_endian = "little")]
impl<T: Digit> TryFrom<f32> for BigUint<T> {
    type Error = FromFloatError<f32>;

    fn try_from(f: f32) -> Result<BigUint<T>, FromFloatError<f32>> {
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
        let ret = BigUint::<T>::from(T::decomposition_from_u32(mantissa));

        Ok(match exponent {
            i if i < 0 => ret >> exponent.abs().try_into().unwrap(),
            i if i > 0 => ret << exponent.try_into().unwrap(),
            _ => ret,
        })
    }
}
