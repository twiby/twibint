use crate::traits::{Digit, DoubleDigit};

use rayon::prelude::*;

pub(super) fn parallel_add_assign<T:Digit>(_rhs: &mut [T], _lhs: &[T]) -> bool {
    false
}
