use crate::digits_vec::Digits;
use core::cmp::Ordering;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigInt {
    pub val: Vec<u32>,
}

impl BigInt {
    pub fn new(val: u32) -> BigInt {
        BigInt { val: vec![val] }
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

impl From<u64> for BigInt {
    fn from(n: u64) -> BigInt {
        let mut ret = BigInt {
            val: vec![
                (n % 4294967296).try_into().unwrap(),
                (n / 4294967296).try_into().unwrap(),
            ],
        };
        ret.remove_trailing_zeros();
        ret
    }
}

impl From<Vec<u32>> for BigInt {
    fn from(v: Vec<u32>) -> BigInt {
        let mut ret = BigInt { val: v };
        ret.remove_trailing_zeros();
        ret
    }
}

impl From<&str> for BigInt {
    fn from(s: &str) -> BigInt {
        let mut ret = BigInt::new(0);

        let mut base = BigInt::new(1);
        for c in s.chars().rev() {
            let v: u32 = c.to_digit(10).unwrap();

            ret += v * &base;
            base *= 10;
        }

        ret.remove_trailing_zeros();
        return ret;
    }
}

impl From<&BigInt> for Digits {
    fn from(b: &BigInt) -> Digits {
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

impl From<&BigInt> for String {
    fn from(b: &BigInt) -> String {
        String::from(&Digits::from(b))
    }
}
impl From<BigInt> for String {
    fn from(b: BigInt) -> String {
        String::from(&Digits::from(&b))
    }
}

impl PartialOrd<BigInt> for BigInt {
    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &BigInt) -> Ordering {
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

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl std::fmt::Binary for BigInt {
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

impl std::fmt::LowerHex for BigInt {
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

impl Default for BigInt {
    fn default() -> BigInt {
        BigInt::new(0)
    }
}

impl std::str::FromStr for BigInt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = BigInt::new(0);

        let mut base = BigInt::new(1);
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

impl std::hash::Hash for BigInt {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.val.hash(state);
    }
}

impl From<&BigInt> for f64 {
    fn from(int: &BigInt) -> f64 {
        let mut base = 1f64;
        let mut ret = 0f64;
        for a in &int.val {
            ret += (*a as f64) * base;
            base *= (1u64 << 32) as f64;
        }
        ret
    }
}

impl std::fmt::LowerExp for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::LowerExp::fmt(&val, f)
    }
}
