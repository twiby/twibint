use crate::BigUint;

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
