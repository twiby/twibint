//! (private) fmt: private module containing implementation of traits
//! pertaining to I/O formatting.

use crate::traits::Digit;
use crate::BigInt;

impl<T: Digit> std::fmt::LowerExp for BigInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::LowerExp::fmt(&val, f)
    }
}
impl<T: Digit> std::fmt::UpperExp for BigInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::UpperExp::fmt(&val, f)
    }
}

impl<T: Digit> std::fmt::LowerHex for BigInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = match self.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&format!("{:x}", self.uint));
        write!(f, "{}", ret)
    }
}
impl<T: Digit> std::fmt::UpperHex for BigInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = match self.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&format!("{:X}", self.uint));
        write!(f, "{}", ret)
    }
}

impl<T: Digit> std::fmt::Display for BigInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl<T: Digit> std::fmt::Binary for BigInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = match self.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&format!("{:b}", self.uint));
        write!(f, "{}", ret)
    }
}
