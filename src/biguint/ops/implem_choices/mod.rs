use crate::traits::Digit;

mod add;
mod mul;
mod sub;

#[cfg(feature = "parallel")]
mod parallel_add;

// Below this number of digits, multiplication is schoolbook
#[cfg(test)]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 2;
#[cfg(test)]
const KARATSUBA_EXTERNAL_THRESHOLD: usize = 2;

#[cfg(not(test))]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 20;
#[cfg(not(test))]
const KARATSUBA_EXTERNAL_THRESHOLD: usize = 500;

const KARATSUBA_EXTERNAL_THRESHOLD_SQUARED: usize =
    KARATSUBA_EXTERNAL_THRESHOLD * KARATSUBA_EXTERNAL_THRESHOLD;

/// Current implementation of multiplication
pub(super) fn mul<T: Digit>(rhs: &[T], lhs: &[T]) -> Vec<T> {
    if rhs.len() * lhs.len() < KARATSUBA_EXTERNAL_THRESHOLD_SQUARED {
        let mut ret = vec![T::ZERO; rhs.len() + lhs.len()];
        mul::schoolbook_add_assign_mul(&mut ret, rhs, lhs);
        return ret;
    }

    mul::karatsuba::<KARATSUBA_INTERNAL_THRESHOLD, _>(rhs, lhs)
}

/// Current implementation of add_assign, returning the carry
/// Assumes rhs has at least the size of lhs
pub(super) fn add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    #[cfg(feature = "parallel")]
    return parallel_add::parallel_add_assign(rhs, lhs);

    #[cfg(not(feature = "parallel"))]
    return add::schoolbook_add_assign(rhs, lhs, false);
}

/// Current implementation of sub_assign
/// Assumes rhs > lhs
pub(super) fn sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) {
    sub::schoolbook_sub_assign(rhs, lhs);
}
