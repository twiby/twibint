use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::errors::{DivisionByZero, FromFloatError, UnexpectedCharacterError};

/// Trait to get the name of an error type as a string, as a static method. \
/// Must be implemented for any error type that will flow into Python.
trait ErrorTypeToString {
    fn str() -> String;
}

/// py_value_error: with an error message as a `&str`, and an error type
/// as a generic parameter, will return a Python `ValueError` with a message
/// containing the original message, plus the error type name.
fn py_value_error<ErrorType: ErrorTypeToString>(msg: &str) -> pyo3::PyErr {
    let mut string = ErrorType::str();
    string.push_str(": ");
    string.push_str(msg);
    PyErr::new::<PyValueError, _>(string)
}

/// Get the `UnexpectedCharacterError` name as a string
impl ErrorTypeToString for UnexpectedCharacterError {
    fn str() -> String {
        "UnexpectedCharacterError: ".to_string()
    }
}
/// Get the `DivisionByZero` name as a string
impl ErrorTypeToString for DivisionByZero {
    fn str() -> String {
        "DivisionByZero: ".to_string()
    }
}
/// Get the `FromFloatError` name as a string
impl<T> ErrorTypeToString for FromFloatError<T> {
    fn str() -> String {
        "FromFloatError: ".to_string()
    }
}

/// Converts a `UnexpectedCharacterError` to a `ValueError`
impl From<UnexpectedCharacterError> for pyo3::PyErr {
    fn from(e: UnexpectedCharacterError) -> Self {
        py_value_error::<DivisionByZero>(&e.to_string())
    }
}
/// Converts a `DivisionByZero` to a `ValueError`
impl From<DivisionByZero> for pyo3::PyErr {
    fn from(e: DivisionByZero) -> Self {
        py_value_error::<DivisionByZero>(&e.to_string())
    }
}
/// Converts a `FromFloatError` to a `ValueError`
impl<T: std::fmt::Display> From<FromFloatError<T>> for pyo3::PyErr {
    fn from(e: FromFloatError<T>) -> Self {
        py_value_error::<FromFloatError<T>>(&e.to_string())
    }
}
