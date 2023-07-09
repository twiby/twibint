//! (private) ops: private module containing all the arithmetic operations'
//! implementations. This is broken down into various submodules.

mod addsub;
mod bitwise;
mod divrem;
mod mul;
mod neg;
mod pow;
mod shift;
mod truediv;

#[cfg(test)]
mod test;
