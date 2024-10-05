//! traits: declares traits for operations not already in `std::ops`.

/// Defines the Digit trait: which defines types that can be used as a digit
mod digit;
pub use digit::Digit;
#[cfg(feature = "unsafe")]
pub(crate) use digit::ToPtr;
pub(crate) use digit::{DoubleDigit, SignedDigit};

/// Defines traits and tests to ensure coherence and completeness of
/// binary op coverage
#[cfg(test)]
mod check_implementations;

/// Shorthand for the result of a division or modulo operation.
/// Is an error in case of a division by zero.
pub type DivisionResult<T> = Result<T, crate::errors::DivisionError>;

/// Trait encapsulating the division and modulo operation as one, as
/// they often go hand in hand.
pub trait RemDiv<T> {
    /// Resulting type of the operation `Self / T`
    type DivOutput;
    /// Resulting type of the operation `Self % T`
    type RemOutput;
    /// Performs the "`RemDiv`" operation, computing both `self / other`
    /// and `self % other`.
    fn rem_div(&self, other: &T) -> DivisionResult<(Self::DivOutput, Self::RemOutput)>;
    /// Performs the divison operation. Default implemention is to call rem_div
    /// and return the relevant item.
    fn div(&self, other: &T) -> DivisionResult<Self::DivOutput> {
        Ok(self.rem_div(other)?.0)
    }
    /// Performs the modulo operation. Default implemention is to call rem_div
    /// and return the relevant item.
    fn rem(&self, other: &T) -> DivisionResult<Self::RemOutput> {
        Ok(self.rem_div(other)?.1)
    }
}

/// Trait for the floating point division: even between 2 BigInt/BigUint, will return
/// a double precision floating point number.
pub trait TrueDiv<T> {
    fn truediv(&self, other: &T) -> DivisionResult<f64>;
}

/// Trait for the operation for raising `self` to the `other`th.
pub trait Pow {
    fn pow(&self, other: usize) -> Self;
}
