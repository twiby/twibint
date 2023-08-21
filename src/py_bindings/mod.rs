//! py_bindings: bindings to build a Python module.
//!
//! This module binds much of the crate's features to Python objects
//! and methods. Rust errors flow as Python errors, with ValueError type.
//! Any function/method called from Python for a BigInt or BigUint object
//! can be called on another object if a BigInt/BigUint can be built
//! from it.
//!
//! For the low-level binding, this module is using the pyo3 framework,
//! which does much of the heavy lifting.

use crate::{BigInt, BigUint};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};

/// Bindings for the BigInt type
mod bigint;
use bigint::PyBigInt;

/// Bindings for the BigUint type
mod biguint;
use biguint::PyBigUint;

/// Bindings for error types
mod errors;

/// This will allow calling a Python method for a BigInt on any object.
/// This trait implentation will then be called to build a BigInt from it.
impl TryFrom<&PyAny> for PyBigInt {
    /// This will be raised as a ValueError in the Python runtime.
    type Error = PyErr;

    /// Operands supported: \
    ///     - python int \
    ///     - python string \
    ///     - BigInt or BigUint \
    ///     - float (32 or 64 bits)
    fn try_from(other: &PyAny) -> PyResult<PyBigInt> {
        // Python int
        if let Ok(int) = other.downcast::<PyInt>() {
            Ok(PyBigInt(int.to_string().as_str().parse()?))
        // Python string
        } else if let Ok(string) = other.downcast::<PyString>() {
            Ok(PyBigInt(string.to_str()?.parse()?))
        // Rust BigInt
        } else if let Ok(int) = other.extract::<PyBigInt>() {
            Ok(int)
        // Rust BigUint
        } else if let Ok(int) = other.extract::<PyBigUint>() {
            Ok(PyBigInt(BigInt::from(int.0)))
        // float 64
        } else if let Ok(float_64) = other.extract::<f64>() {
            Ok(PyBigInt(float_64.try_into()?))
        // float 32
        } else if let Ok(float_32) = other.extract::<f32>() {
            Ok(PyBigInt(float_32.try_into()?))
        } else {
            Err(PyErr::new::<PyValueError, _>("Object of unsupported type"))
        }
    }
}

/// This will allow calling a Python method for a BigUint on any object.
/// This trait implentation will then be called to build a BigUint from it.
impl TryFrom<&PyAny> for PyBigUint {
    /// This will be raised as a ValueError in the Python runtime.
    type Error = PyErr;

    /// Operands supported: \
    ///     - Any supported by BigInt, if the produced BigInt is positive.
    fn try_from(other: &PyAny) -> PyResult<PyBigUint> {
        match PyBigInt::try_from(other)? {
            PyBigInt(BigInt { sign: true, uint }) => Ok(PyBigUint(uint)),
            _ => Err(PyErr::new::<PyValueError, _>(
                "Cannot create PyBigUint from negative integer",
            )),
        }
    }
}

impl BigUint<u64> {
    /// Python binding to convert to a int
    // TODO: not efficient at all, fail between 10000 and 100000 digits
    fn __int__(&self, py: Python<'_>) -> PyResult<PyObject> {
        // Lowest bits
        let mut py_obj = self.val[0].to_object(py);

        // Move to highest bits, shifting and adding each time
        for (i, val) in self.val.iter().enumerate().skip(1) {
            let lhs = val
                .to_object(py)
                .call_method1(py, "__lshift__", (64 * i,))?;
            py_obj = py_obj.call_method1(py, "__or__", (lhs,))?;
        }
        Ok(py_obj)
    }
}

#[pyfunction]
fn gen_random_pybiguint(n: usize) -> PyBigUint {
    PyBigUint(crate::gen_random_biguint(n * 64))
}

/// Declaring our Python module.
///
/// This module will contain 2 classes: \
///     - PyBigUint \
///     - PyBigInt
///
/// This module contains 1 function: \
///     - gen_random_pybiguint
#[pymodule]
fn twibint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBigUint>()?;
    m.add_class::<PyBigInt>()?;
    m.add_function(wrap_pyfunction!(gen_random_pybiguint, m)?)?;
    return Ok(());
}
