//! (private) froms: private module containing all`From<T>`implementations
//!
//! These implementations are meant to be the main way to construct a BigInt,
//! or export its value into another type.

use crate::errors::FromFloatError;
use crate::errors::UnexpectedCharacterError;
use crate::{BigInt, BigUint};

impl From<i32> for BigInt {
    fn from(val: i32) -> BigInt {
        BigInt::new(val)
    }
}
impl From<u32> for BigInt {
    fn from(val: u32) -> BigInt {
        BigInt::from(BigUint::from(val))
    }
}
impl From<u64> for BigInt {
    fn from(val: u64) -> BigInt {
        BigInt {
            uint: BigUint::from(val),
            sign: true,
        }
    }
}
impl From<i64> for BigInt {
    fn from(val: i64) -> BigInt {
        BigInt {
            uint: BigUint::from(TryInto::<u64>::try_into(val.abs()).unwrap()),
            sign: val.is_positive(),
        }
    }
}

impl From<BigUint> for BigInt {
    fn from(val: BigUint) -> BigInt {
        BigInt {
            uint: val,
            sign: true,
        }
    }
}

impl From<Vec<u32>> for BigInt {
    fn from(val: Vec<u32>) -> BigInt {
        BigInt::from(BigUint::from(val))
    }
}

impl From<&BigInt> for f64 {
    fn from(int: &BigInt) -> f64 {
        match int.sign {
            true => f64::from(&int.uint),
            false => -f64::from(&int.uint),
        }
    }
}

impl From<&BigInt> for f32 {
    fn from(int: &BigInt) -> f32 {
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

impl std::str::FromStr for BigInt {
    type Err = UnexpectedCharacterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(BigInt::default());
        }

        let (r, s2) = car_cdr(s);
        match r {
            '-' => {
                let mut ret = BigInt::from(s2.parse::<BigUint>()?);
                ret.sign = false;
                Ok(ret)
            }
            _ => Ok(BigInt::from(s.parse::<BigUint>()?)),
        }
    }
}

impl From<&BigInt> for String {
    fn from(b: &BigInt) -> String {
        let mut ret = match b.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&String::from(&b.uint));
        ret
    }
}

impl From<BigInt> for String {
    fn from(b: BigInt) -> String {
        String::from(&b)
    }
}

impl From<&str> for BigInt {
    fn from(s: &str) -> BigInt {
        <BigInt as std::str::FromStr>::from_str(s).unwrap()
    }
}

#[cfg(target_endian = "little")]
impl TryFrom<f64> for BigInt {
    type Error = FromFloatError<f64>;

    fn try_from(f: f64) -> Result<BigInt, FromFloatError<f64>> {
        if f != 0f64 && !f.is_normal() {
            return Err(FromFloatError::NotNormal(f));
        }

        if f < 0.0 {
            let mut ret = BigInt::from(BigUint::try_from(-f)?);
            ret.sign = false;
            Ok(ret)
        } else {
            Ok(BigInt::from(BigUint::try_from(f)?))
        }
    }
}

#[cfg(target_endian = "little")]
impl TryFrom<f32> for BigInt {
    type Error = FromFloatError<f32>;

    fn try_from(f: f32) -> Result<BigInt, FromFloatError<f32>> {
        if f != 0f32 && !f.is_normal() {
            return Err(FromFloatError::NotNormal(f));
        }

        if f < 0.0 {
            let mut ret = BigInt::from(BigUint::try_from(-f)?);
            ret.sign = false;
            Ok(ret)
        } else {
            Ok(BigInt::from(BigUint::try_from(f)?))
        }
    }
}
