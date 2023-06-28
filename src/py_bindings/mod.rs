use crate::{BigInt, BigUint};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

mod bigsint;
mod biguint;
mod errors;

impl TryFrom<&PyAny> for BigInt {
    type Error = PyErr;
    fn try_from(other: &PyAny) -> PyResult<BigInt> {
        if let Ok(int) = other.downcast::<PyInt>() {
            Ok(BigInt::__init__(int)?)
        } else if let Ok(int) = other.extract::<BigInt>() {
            Ok(int)
        } else if let Ok(float_64) = other.extract::<f64>() {
            Ok(float_64.try_into()?)
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
        if let Ok(int) = other.downcast::<PyInt>() {
            Ok(BigUint::__init__(int)?)
        } else if let Ok(int) = other.extract::<BigUint>() {
            Ok(int)
        } else if let Ok(float_64) = other.extract::<f64>() {
            Ok(float_64.try_into()?)
        } else if let Ok(float_32) = other.extract::<f32>() {
            Ok(float_32.try_into()?)
        } else {
            Err(PyErr::new::<PyValueError, _>("Object of unsupported type"))
        }
    }
}

#[pymodule]
fn bigint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<biguint::BigUint>()?;
    m.add_class::<bigsint::BigInt>()?;
    return Ok(());
}
