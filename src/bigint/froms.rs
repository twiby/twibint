use crate::{BigInt, BigUint};

impl From<i32> for BigInt {
    fn from(val: i32) -> BigInt {
        BigInt::new(val)
    }
}
impl From<u32> for BigInt {
    fn from(val: u32) -> BigInt {
        BigInt::new(val as i32)
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
