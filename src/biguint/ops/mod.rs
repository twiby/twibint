//! (private) ops: private module containing all the arithmetic operations'
//! implementations. This is broken down into various submodules.

pub(crate) mod addsub;
pub(crate) mod bitwise;
pub(crate) mod divrem;
pub(crate) mod mul;
pub(crate) mod pow;
pub(crate) mod shift;
pub(crate) mod truediv;

mod implem_choices;

#[cfg(test)]
mod test;
