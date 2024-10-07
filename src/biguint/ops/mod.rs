//! (private) ops: private module containing all the arithmetic operations'
//! implementations. This is broken down into various submodules.

pub(crate) mod addsub;
pub(crate) mod bitwise;
pub(crate) mod divrem;
pub(crate) mod mul;
pub(crate) mod pow;
pub(crate) mod shift;
pub(crate) mod truediv;

mod algorithms;
pub(crate) use algorithms::add_assign;
pub(crate) use algorithms::div;
pub(crate) use algorithms::mul;
pub(crate) use algorithms::mul_assign_digit;
pub(crate) use algorithms::rsub_assign;
pub(crate) use algorithms::sub_assign;

#[cfg(test)]
mod test;
