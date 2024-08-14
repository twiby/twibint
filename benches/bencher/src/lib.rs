#![allow(unused_imports)]
use rand::distributions::Standard;
use rand::prelude::*;

#[cfg(not(any(feature = "twibint", feature = "num-bigint")))]
compile_error!("Exactly one feature must be used");

#[cfg(all(feature = "twibint", feature = "num-bigint"))]
compile_error!("Exactly one feature must be used");

#[cfg(feature = "twibint")]
pub fn gen_random_biguint(n: usize) -> twibint::BigUint<u64> {
    let ret = twibint::gen_random_biguint::<u64>(n);
    assert_eq!(ret.nb_bits(), n);
    ret
}

#[cfg(feature = "num-bigint")]
pub fn gen_random_biguint(n: usize) -> num_bigint::BigUint {
    use num_bigint::RandBigInt;

    let n: u64 = n.try_into().unwrap();

    let mut rng = rand::thread_rng();
    let mut ret = rng.gen_biguint(n);
    let nb_bits = ret.bits();

    if nb_bits == 0 {
        ret = num_bigint::BigUint::from(1u32) << (n - 1);
    } else if nb_bits > n {
        ret >>= nb_bits - n;
    } else if nb_bits < n {
        ret <<= n - nb_bits;
    }

    assert_eq!(ret.bits(), n);
    ret
}

pub trait GetNbBits {
    fn get_nb_bits(&self) -> usize;
}

#[cfg(feature = "twibint")]
impl GetNbBits for twibint::BigUint<u64> {
    fn get_nb_bits(&self) -> usize {
        self.nb_bits()
    }
}

#[cfg(feature = "num-bigint")]
impl GetNbBits for num_bigint::BigUint {
    fn get_nb_bits(&self) -> usize {
        self.bits().try_into().unwrap()
    }
}
