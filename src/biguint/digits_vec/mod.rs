//! (private) digits_vec: private module for a base-10 representation of unsigned
//! integers. Useful for converting to or from strings.

#[cfg(test)]
mod test;

/// Private structure only to handle a few situations on base 10 digits. \
/// Useful when producing/handling strings of human readable base 10 digits
pub(crate) struct Digits {
    val: Vec<u8>,
}

impl Digits {
    /// Constructor from native unsigned integers
    pub(crate) fn new(mut val: usize) -> Digits {
        let mut ret = Digits { val: Vec::new() };

        while val > 0 {
            ret.val.push((val % 10).try_into().unwrap());

            val -= *ret.val.iter().last().unwrap() as usize;
            val /= 10;
        }

        return ret;
    }

    /// Add the value n to the kth digit
    ///
    /// Assumes n belong to 0..10, and k is lower than or equal to
    /// the number of base-10 digits.
    pub(crate) fn add_n_at_k(&mut self, n: u8, k: usize) {
        if n == 0 {
            return;
        } else if k == self.val.len() {
            self.val.push(n);
        } else {
            let new_v = self.val[k] + n;
            self.val[k] = new_v % 10;
            self.add_n_at_k((new_v - self.val[k]) / 10, k + 1);
        }
    }

    /// Multiplies the integer by 2
    pub(crate) fn times_2(&mut self) {
        let digits = self.val.clone();
        for i in 0..digits.len() {
            self.add_n_at_k(digits[i], i);
        }
    }
}

impl From<&str> for Digits {
    fn from(other: &str) -> Digits {
        other.parse().unwrap()
    }
}

impl std::str::FromStr for Digits {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Digits::new(0);

        for c in s.chars().rev() {
            ret.val.push(match c.to_digit(10) {
                Some(uint) => uint.try_into().unwrap(),
                None => return Err(()),
            });
        }

        Ok(ret)
    }
}

impl From<&Digits> for String {
    fn from(other: &Digits) -> String {
        if other.val.len() == 0 {
            return "0".to_string();
        } else {
            let mut ret = "".to_string();
            for c in other.val.iter().rev() {
                ret.push(char::from_digit((*c).into(), 10).unwrap());
            }
            return ret;
        }
    }
}
