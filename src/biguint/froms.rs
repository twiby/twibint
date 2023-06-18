use crate::biguint::Digits;
use crate::BigUint;

impl From<&BigUint> for f64 {
    fn from(int: &BigUint) -> f64 {
        let mut base = 1f64;
        let mut ret = 0f64;
        for a in &int.val {
            ret += (*a as f64) * base;
            base *= (1u64 << 32) as f64;
        }
        ret
    }
}

impl From<&BigUint> for f32 {
    fn from(int: &BigUint) -> f32 {
        let mut base = 1f32;
        let mut ret = 0f32;
        for a in &int.val {
            ret += (*a as f32) * base;
            base *= (1u64 << 32) as f32;
        }
        ret
    }
}

#[derive(Debug)]
pub struct UnexpectedCharacterError(char);

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
            base *= 10;
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
        <BigUint as std::str::FromStr>::from_str(s).unwrap()
    }
}

impl From<u64> for BigUint {
    fn from(n: u64) -> BigUint {
        let mut ret = BigUint {
            val: vec![
                (n % 4294967296).try_into().unwrap(),
                (n / 4294967296).try_into().unwrap(),
            ],
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
pub struct IntoU64Error {}
impl TryFrom<&BigUint> for u64 {
    type Error = IntoU64Error;
    fn try_from(uint: &BigUint) -> Result<u64, IntoU64Error> {
        match uint.val.len() {
            0 => unreachable!(),
            1 => Ok(uint.val[0].into()),
            2 => Ok(uint.val[0] as u64 + ((uint.val[1] as u64) << 32)),
            _ => Err(IntoU64Error {}),
        }
    }
}
impl TryFrom<BigUint> for u64 {
    type Error = IntoU64Error;
    fn try_from(uint: BigUint) -> Result<u64, IntoU64Error> {
        u64::try_from(&uint)
    }
}

#[derive(Debug)]
pub enum FromFloatError<T> {
    NotNormal(T),
    Negative(T),
}

impl<T: std::fmt::Display> std::fmt::Display for FromFloatError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FromFloatError::NotNormal(num) => {
                write!(f, "Attempt at converting an abnormal float: {}", num)
            }
            FromFloatError::Negative(num) => {
                write!(f, "Attempt at converting a negative number: {}", num)
            }
        }
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
