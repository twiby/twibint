//! errors: declares all error types used throughout the crate

/// UnexpectedCharacterError: might be returned when building
/// from a string.
#[derive(Debug)]
pub struct UnexpectedCharacterError(pub char);

/// FromFloatError: might be returned when building
/// from a float.
#[derive(Debug)]
pub enum FromFloatError<T> {
    NotNormal(T),
    Negative(T),
}

/// DivisionByZero: might be returned when calling the division
/// or rem operations
#[derive(Debug)]
pub struct DivisionByZero();

impl<T: std::fmt::Display> std::fmt::Display for FromFloatError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FromFloatError::NotNormal(num) => {
                write!(f, "Attempt at converting an abnormal float: {}", num)
            }
            FromFloatError::Negative(num) => {
                write!(f, "Attempt at converting a negative number: {}", num)
            }
        }
    }
}

impl std::fmt::Display for UnexpectedCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Got an unexpected character when reading string: {}",
            self.0
        )
    }
}

impl std::fmt::Display for DivisionByZero {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Attempt at division by zero !")
    }
}
