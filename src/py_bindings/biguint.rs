use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct BigUint {
    val: crate::BigUint,
}
#[pymethods]
impl BigUint {
    #[new]
    pub fn new(n: u64) -> PyResult<Self> {
        Ok(Self {
            val: crate::BigUint::from(n),
        })
    }
    pub fn __abs__(&self) -> PyResult<Self> {
        Ok(self.clone())
    }
    pub fn __add__(&self, other: &Self) -> PyResult<Self> {
        Ok(Self {
            val: &self.val + &other.val,
        })
    }
    pub fn __and__(&self, other: &Self) -> PyResult<Self> {
        Ok(Self {
            val: &self.val & &other.val,
        })
    }
    pub fn __mul__(&self, other: &Self) -> PyResult<Self> {
        Ok(Self {
            val: &self.val * &other.val,
        })
    }

    pub fn __str__(&self) -> PyResult<String> {
        return Ok(self.val.to_string());
    }
    pub fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("{:?}", self.val));
    }
}

impl From<crate::BigUint> for BigUint {
    fn from(c: crate::BigUint) -> Self {
        Self { val: c }
    }
}
impl From<BigUint> for crate::BigUint {
    fn from(c: BigUint) -> Self {
        c.val
    }
}
