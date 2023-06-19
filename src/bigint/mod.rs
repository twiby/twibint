use crate::BigUint;
use core::cmp::Ordering;

use pyo3::prelude::*;

#[macro_export]
macro_rules! bigint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigInt::from($x)
            )*
        }
    };
}

#[macro_export]
macro_rules! bigintvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            BigInt::from(temp_vec)
        }
    };
}

mod fmt;
mod froms;
mod ops;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Eq)]
#[pyclass]
pub struct BigInt {
    pub uint: BigUint,
    pub sign: bool,
}

impl BigInt {
    pub fn new(val: i32) -> BigInt {
        BigInt {
            uint: BigUint::new(val.abs().try_into().unwrap()),
            sign: val.is_positive(),
        }
    }

    pub fn is_sign_positive(&self) -> bool {
        self.uint != Default::default() && self.sign
    }
    pub fn is_sign_negative(&self) -> bool {
        self.uint != Default::default() && self.sign
    }
}

impl Default for BigInt {
    fn default() -> BigInt {
        BigInt {
            uint: Default::default(),
            sign: true,
        }
    }
}

impl std::hash::Hash for BigInt {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.uint.hash(state);
        self.sign.hash(state);
    }
}

impl PartialOrd<BigInt> for BigInt {
    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &BigInt) -> Ordering {
        match (self.sign, other.sign) {
            (true, true) => self.uint.cmp(&other.uint),
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => self.uint.cmp(&other.uint).reverse(),
        }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.uint == other.uint && ((self.sign == other.sign) || (self.uint.val == vec![0]))
    }
}
