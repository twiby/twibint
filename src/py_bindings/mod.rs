use pyo3::prelude::*;

mod biguint;

#[pymodule]
fn bigint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<biguint::BigUint>()?;
    return Ok(());
}
