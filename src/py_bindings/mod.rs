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

/// Bindings for the BigInt type
mod bigint_py;

/// Bindings for the BigUint type
mod biguint_py;

/// Bindings for error types
mod errors;

#[pyo3::pymodule]
pub mod twibint {
    use super::bigint_py;
    use super::biguint_py;

    #[pymodule_export]
    pub use bigint_py::BigInt;
    #[pymodule_export]
    pub use biguint_py::BigUint;

    use crate::Imported;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use pyo3::types::{PyInt, PyString};
    use pyo3::IntoPyObjectExt;

    /// This will allow calling a Python method for a BigInt on any object.
    /// This trait implentation will then be called to build a BigInt from it.
    impl TryFrom<Bound<'_, PyAny>> for BigInt {
        /// This will be raised as a ValueError in the Python runtime.
        type Error = PyErr;

        /// Operands supported: \
        ///     - python int \
        ///     - python string \
        ///     - BigInt or BigUint \
        ///     - float (32 or 64 bits)
        fn try_from(other: Bound<'_, PyAny>) -> PyResult<BigInt> {
            // Python int
            if let Ok(int) = Bound::cast::<PyInt>(&other) {
                Ok(BigInt(int.to_string().as_str().parse()?))
            // Python string
            } else if let Ok(string) = Bound::cast::<PyString>(&other) {
                Ok(BigInt(string.to_str()?.parse()?))
            // Rust BigInt
            } else if let Ok(int) = other.extract::<BigInt>() {
                Ok(int)
            // Rust BigUint
            } else if let Ok(int) = other.extract::<BigUint>() {
                Ok(BigInt(crate::BigInt::from(int.0)))
            // float 64
            } else if let Ok(float_64) = other.extract::<f64>() {
                Ok(BigInt(float_64.try_into()?))
            // float 32
            } else if let Ok(float_32) = other.extract::<f32>() {
                Ok(BigInt(float_32.try_into()?))
            } else {
                Err(PyErr::new::<PyValueError, _>("Object of unsupported type"))
            }
        }
    }

    /// This will allow calling a Python method for a BigUint on any object.
    /// This trait implentation will then be called to build a BigUint from it.
    impl TryFrom<Bound<'_, PyAny>> for BigUint {
        /// This will be raised as a ValueError in the Python runtime.
        type Error = PyErr;

        /// Operands supported: \
        ///     - Any supported by BigInt, if the produced BigInt is positive.
        fn try_from(other: Bound<'_, PyAny>) -> PyResult<BigUint> {
            match BigInt::try_from(other)? {
                BigInt(crate::BigInt { sign: true, uint }) => Ok(BigUint(uint)),
                _ => Err(PyErr::new::<PyValueError, _>(
                    "Cannot create BigUint from negative integer",
                )),
            }
        }
    }

    impl crate::BigUint<u64> {
        /// Python binding to convert to a int
        // TODO: not efficient at all, fail between 10000 and 100000 digits
        pub fn __int__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
            // Lowest bits
            let mut py_obj: Bound<'_, PyAny> = self.val[0].into_bound_py_any(py)?;

            // Move to highest bits, shifting and adding each time
            for (i, val) in self.val.iter().enumerate().skip(1) {
                let lhs = val
                    .into_pyobject(py)?
                    .call_method1("__lshift__", (64 * i,))?;
                py_obj = py_obj.call_method1("__or__", (lhs,))?;
            }
            Ok(py_obj)
        }
    }

    #[cfg(feature = "rand")]
    #[pyfunction]
    fn gen_random_biguint(n: usize) -> BigUint {
        BigUint(crate::gen_random_biguint(n * 64))
    }

    #[pyfunction]
    fn read_from_file<'py>(path: String, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(match Imported::<u64>::read_from_file(&path)? {
            Imported::Uint(uint) => BigUint(uint).into_bound_py_any(py)?,
            Imported::Int(int) => BigInt(int).into_bound_py_any(py)?,
        })
    }
}
