#[derive(Debug)]
pub struct UnexpectedCharacterError(pub char);

#[derive(Debug)]
pub enum FromFloatError<T> {
    NotNormal(T),
    Negative(T),
}

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
