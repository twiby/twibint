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

impl std::str::FromStr for BigUint {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = BigUint::new(0);

        let mut base = BigUint::new(1);
        for c in s.chars().rev() {
            let v: u32 = match c.to_digit(10) {
                Some(val) => val,
                None => return Err(()),
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
        let mut ret = BigUint::new(0);

        let mut base = BigUint::new(1);
        for c in s.chars().rev() {
            let v: u32 = c.to_digit(10).unwrap();

            ret += v * &base;
            base *= 10;
        }

        ret.remove_trailing_zeros();
        return ret;
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
