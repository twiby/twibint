use pyo3::prelude::*;
use pyo3::pyclass::CompareOp::*;

pub use crate::BigUint;

#[pymethods]
impl BigUint {
    #[new]
    pub fn __init__(n: &pyo3::types::PyInt) -> PyResult<Self> {
        Ok(n.to_string().as_str().parse()?)
    }
    pub fn __add__(&self, other: &Self) -> Self {
        self + other
    }

    pub fn __richcmp__(&self, other: &Self, cmp: pyo3::basic::CompareOp) -> bool {
        match cmp {
            Lt => self < other,
            Le => self <= other,
            Eq => self == other,
            Ne => self != other,
            Gt => self > other,
            Ge => self >= other,
        }
    }

    pub fn __bool__(&self) -> bool {
        self != &BigUint::default()
    }

    pub fn __str__(&self) -> PyResult<String> {
        return Ok(self.to_string());
    }
    pub fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("{:?}", self));
    }
}
