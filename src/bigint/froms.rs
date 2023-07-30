//! (private) froms: private module containing all`From<T>`implementations
//!
//! These implementations are meant to be the main way to construct a BigInt,
//! or export its value into another type.

use crate::errors::FromFloatError;
use crate::errors::UnexpectedCharacterError;
use crate::traits::Digit;
use crate::{BigInt, BigUint};

impl<T: Digit> From<i32> for BigInt<T> {
    fn from(val: i32) -> BigInt<T> {
        BigInt::<T> {
            uint: BigUint::<T>::from(TryInto::<u32>::try_into(val.abs()).unwrap()),
            sign: val.is_positive(),
        }
    }
}
impl<T: Digit> From<u32> for BigInt<T> {
    fn from(val: u32) -> BigInt<T> {
        BigInt::<T>::from(BigUint::<T>::from(val))
    }
}
impl<T: Digit> From<u64> for BigInt<T> {
    fn from(val: u64) -> BigInt<T> {
        BigInt::<T>::from(BigUint::<T>::from(val))
    }
}
impl<T: Digit> From<i64> for BigInt<T> {
    fn from(val: i64) -> BigInt<T> {
        BigInt::<T> {
            uint: BigUint::<T>::from(TryInto::<u64>::try_into(val.abs()).unwrap()),
            sign: val.is_positive(),
        }
    }
}

impl<T: Digit> From<BigUint<T>> for BigInt<T> {
    fn from(val: BigUint<T>) -> BigInt<T> {
        BigInt::<T> {
            uint: val,
            sign: true,
        }
    }
}

impl<T: Digit> From<Vec<T>> for BigInt<T> {
    fn from(val: Vec<T>) -> BigInt<T> {
        BigInt::<T>::from(BigUint::<T>::from(val))
    }
}

impl<T: Digit> From<&BigInt<T>> for f64 {
    fn from(int: &BigInt<T>) -> f64 {
        match int.sign {
            true => f64::from(&int.uint),
            false => -f64::from(&int.uint),
        }
    }
}

impl<T: Digit> From<&BigInt<T>> for f32 {
    fn from(int: &BigInt<T>) -> f32 {
        match int.sign {
            true => f32::from(&int.uint),
            false => -f32::from(&int.uint),
        }
    }
}

fn car_cdr(s: &str) -> (char, &str) {
    assert!(s.len() > 0);

    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x.chars().next().unwrap(), &s[i..]),
            None => (),
        }
    }

    (s[0..0].chars().next().unwrap(), s)
}

impl<T: Digit> std::str::FromStr for BigInt<T> {
    type Err = UnexpectedCharacterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(BigInt::<T>::default());
        }

        let (r, s2) = car_cdr(s);
        match r {
            '-' => Ok(BigInt::<T> {
                uint: s2.parse::<BigUint<T>>()?,
                sign: false,
            }),
            _ => Ok(BigInt::<T>::from(s.parse::<BigUint<T>>()?)),
        }
    }
}

impl<T: Digit> From<&BigInt<T>> for String {
    fn from(b: &BigInt<T>) -> String {
        let mut ret = match b.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&String::from(&b.uint));
        ret
    }
}

impl<T: Digit> From<BigInt<T>> for String {
    fn from(b: BigInt<T>) -> String {
        String::from(&b)
    }
}

impl<T: Digit> From<&str> for BigInt<T> {
    fn from(s: &str) -> BigInt<T> {
        <BigInt<T> as std::str::FromStr>::from_str(s).unwrap()
    }
}

#[cfg(target_endian = "little")]
impl<T: Digit> TryFrom<f64> for BigInt<T> {
    type Error = FromFloatError<f64>;

    fn try_from(f: f64) -> Result<BigInt<T>, FromFloatError<f64>> {
        if f != 0f64 && !f.is_normal() {
            return Err(FromFloatError::NotNormal(f));
        }

        if f < 0.0 {
            Ok(BigInt::<T> {
                uint: BigUint::<T>::try_from(-f)?,
                sign: false,
            })
        } else {
            Ok(BigInt::<T>::from(BigUint::<T>::try_from(f)?))
        }
    }
}

#[cfg(target_endian = "little")]
impl<T: Digit> TryFrom<f32> for BigInt<T> {
    type Error = FromFloatError<f32>;

    fn try_from(f: f32) -> Result<BigInt<T>, FromFloatError<f32>> {
        if f != 0f32 && !f.is_normal() {
            return Err(FromFloatError::NotNormal(f));
        }

        if f < 0.0 {
            Ok(BigInt::<T> {
                uint: BigUint::<T>::try_from(-f)?,
                sign: false,
            })
        } else {
            Ok(BigInt::<T>::from(BigUint::<T>::try_from(f)?))
        }
    }
}
