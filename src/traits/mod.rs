//! traits: declares traits for operations not already in `std::ops`.

/// Defines the Digit trait: which defines types that can be used as a digit
mod digit;
pub use digit::Digit;
pub(crate) use digit::{DoubleDigit, SignedDigit};

/// Shorthand for the result of a division or modulo operation.
/// Is an error in case of a division by zero.
pub type DivisionResult<T> = Result<T, crate::errors::DivisionByZero>;

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
        self.rem_div(other).map(|ret| ret.0)
    }
    /// Performs the modulo operation. Default implemention is to call rem_div
    /// and return the relevant item.
    fn rem(&self, other: &T) -> DivisionResult<Self::RemOutput> {
        self.rem_div(other).map(|ret| ret.1)
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
