//! (private) fmt: private module containing implementation of traits
//! pertaining to I/O formatting.

use crate::traits::Digit;
use crate::BigUint;

// TODO: LowerExp and UpperExp could actually be implemented for values
// outside the range of a f64 (not sure of the actual use case).
impl<T: Digit> std::fmt::LowerExp for BigUint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::LowerExp::fmt(&val, f)
    }
}
impl<T: Digit> std::fmt::UpperExp for BigUint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::UpperExp::fmt(&val, f)
    }
}

impl<T: Digit> std::fmt::LowerHex for BigUint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for c in self.val.iter().rev() {
            let binary = &format!("{:x}", c);
            let mut full_binary = "".to_string();
            for _ in 0..(T::NB_BITS / 4) - binary.len() {
                full_binary.push('0');
            }
            full_binary.push_str(&binary);
            ret.push_str(&full_binary);
        }

        write!(f, "{}", ret)
    }
}
impl<T: Digit> std::fmt::UpperHex for BigUint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for c in self.val.iter().rev() {
            let binary = &format!("{:X}", c);
            let mut full_binary = "".to_string();
            for _ in 0..(T::NB_BITS / 4) - binary.len() {
                full_binary.push('0');
            }
            full_binary.push_str(&binary);
            ret.push_str(&full_binary);
        }

        write!(f, "{}", ret)
    }
}

impl<T: Digit> std::fmt::Display for BigUint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl<T: Digit> std::fmt::Binary for BigUint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for c in self.val.iter().rev() {
            let binary = &format!("{:b}", c);
            let mut full_binary = "".to_string();
            for _ in 0..T::NB_BITS - binary.len() {
                full_binary.push('0');
            }
            full_binary.push_str(&binary);
            ret.push_str(&full_binary);
        }

        write!(f, "{}", ret)
    }
}
