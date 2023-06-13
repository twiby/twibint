use core::cmp::Ordering;
use digits_vec::Digits;

#[macro_export]
macro_rules! biguintvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            BigUint::from(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! biguint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigUint::from($x)
            )*
        }
    };
}

mod digits_vec;
mod ops;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigUint {
    pub val: Vec<u32>,
}

impl BigUint {
    pub fn new(val: u32) -> BigUint {
        BigUint { val: vec![val] }
    }

    #[inline]
    pub fn nb_bits(&self) -> usize {
        32 * self.val.len()
    }

    #[inline]
    pub fn bit(&self, b: usize) -> bool {
        (self.val[b / 32] >> b % 32) & 1 != 0
    }

    #[inline]
    pub fn bits<'a>(&'a self) -> impl DoubleEndedIterator<Item = bool> + 'a {
        (0..self.nb_bits()).map(|b| self.bit(b))
    }

    #[inline]
    pub(crate) fn remove_trailing_zeros(&mut self) {
        while self.val.len() > 1 && self.val.last() == Some(&0) {
            self.val.pop();
        }
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

impl PartialOrd<BigUint> for BigUint {
    fn partial_cmp(&self, other: &BigUint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &BigUint) -> Ordering {
        match self.val.len().cmp(&other.val.len()) {
            Ordering::Equal => (),
            o => return o,
        };
        for (a, b) in self.val.iter().zip(other.val.iter()).rev() {
            match a.cmp(b) {
                Ordering::Equal => continue,
                o => return o,
            };
        }
        Ordering::Equal
    }
}

impl std::fmt::Display for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl std::fmt::Binary for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for c in self.val.iter().rev() {
            let binary = &format!("{:b}", c);
            let mut full_binary = "".to_string();
            for _ in 0..32 - binary.len() {
                full_binary.push('0');
            }
            full_binary.push_str(&binary);
            ret.push_str(&full_binary);
        }

        write!(f, "{}", ret)
    }
}

impl std::fmt::LowerHex for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for c in self.val.iter().rev() {
            let binary = &format!("{:x}", c);
            let mut full_binary = "".to_string();
            for _ in 0..8 - binary.len() {
                full_binary.push('0');
            }
            full_binary.push_str(&binary);
            ret.push_str(&full_binary);
        }

        write!(f, "{}", ret)
    }
}
impl std::fmt::UpperHex for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for c in self.val.iter().rev() {
            let binary = &format!("{:X}", c);
            let mut full_binary = "".to_string();
            for _ in 0..8 - binary.len() {
                full_binary.push('0');
            }
            full_binary.push_str(&binary);
            ret.push_str(&full_binary);
        }

        write!(f, "{}", ret)
    }
}

impl Default for BigUint {
    fn default() -> BigUint {
        BigUint::new(0)
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

impl std::hash::Hash for BigUint {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.val.hash(state);
    }
}

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

impl std::fmt::LowerExp for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::LowerExp::fmt(&val, f)
    }
}
impl std::fmt::UpperExp for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::UpperExp::fmt(&val, f)
    }
}
