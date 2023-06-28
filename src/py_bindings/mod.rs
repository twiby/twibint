use crate::{BigInt, BigUint};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};

mod bigsint;
mod biguint;
mod errors;

impl TryFrom<&PyAny> for BigInt {
    type Error = PyErr;
    fn try_from(other: &PyAny) -> PyResult<BigInt> {
        // Python int
        if let Ok(int) = other.downcast::<PyInt>() {
            Ok(int.to_string().as_str().parse()?)
        // Python string
        } else if let Ok(string) = other.downcast::<PyString>() {
            Ok(string.to_str()?.parse()?)
        // Rust BigInt
        } else if let Ok(int) = other.extract::<BigInt>() {
            Ok(int)
        // Rust BigUint
        } else if let Ok(int) = other.extract::<BigUint>() {
            Ok(BigInt::from(int))
        // float 64
        } else if let Ok(float_64) = other.extract::<f64>() {
            Ok(float_64.try_into()?)
        // float 32
        } else if let Ok(float_32) = other.extract::<f32>() {
            Ok(float_32.try_into()?)
        } else {
            Err(PyErr::new::<PyValueError, _>("Object of unsupported type"))
        }
    }
}

impl TryFrom<&PyAny> for BigUint {
    type Error = PyErr;
    fn try_from(other: &PyAny) -> PyResult<BigUint> {
        match BigInt::try_from(other)? {
            BigInt { sign: true, uint } => Ok(uint),
            _ => Err(PyErr::new::<PyValueError, _>(
                "Cannot create BigUint from negative integer",
            )),
        }
    }
}

#[pymodule]
fn bigint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<biguint::BigUint>()?;
    m.add_class::<bigsint::BigInt>()?;
    return Ok(());
}
