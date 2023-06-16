use crate::biguint::froms::FromFloatError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::biguint::froms::UnexpectedCharacterError;

trait ErrorTypeToString {
    fn str() -> String;
}

fn py_value_error<ErrorType: ErrorTypeToString>(msg: &str) -> pyo3::PyErr {
    let mut string = ErrorType::str();
    string.push_str(": ");
    string.push_str(msg);
    PyErr::new::<PyValueError, _>(string)
}

impl ErrorTypeToString for UnexpectedCharacterError {
    fn str() -> String {
        "UnexpectedCharacterError: ".to_string()
    }
}
impl From<UnexpectedCharacterError> for pyo3::PyErr {
    fn from(e: UnexpectedCharacterError) -> Self {
        py_value_error::<UnexpectedCharacterError>(format!("{:?}", e).as_str())
    }
}

impl<T> ErrorTypeToString for FromFloatError<T> {
    fn str() -> String {
        "ModularArithmeticError: ".to_string()
    }
}
impl<T: std::fmt::Display> From<FromFloatError<T>> for pyo3::PyErr {
    fn from(e: FromFloatError<T>) -> Self {
        py_value_error::<FromFloatError<T>>(&e.to_string())
    }
}
