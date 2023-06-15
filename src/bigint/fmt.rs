use crate::BigInt;

impl std::fmt::LowerExp for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::LowerExp::fmt(&val, f)
    }
}
impl std::fmt::UpperExp for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = f64::from(self);
        std::fmt::UpperExp::fmt(&val, f)
    }
}

impl std::fmt::LowerHex for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = match self.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&format!("{:x}", self.uint));
        write!(f, "{}", ret)
    }
}
impl std::fmt::UpperHex for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = match self.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&format!("{:X}", self.uint));
        write!(f, "{}", ret)
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl std::fmt::Binary for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = match self.sign {
            true => "".to_string(),
            false => "-".to_string(),
        };
        ret.push_str(&format!("{:b}", self.uint));
        write!(f, "{}", ret)
    }
}
