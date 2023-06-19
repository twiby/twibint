use pyo3::prelude::*;

mod bigsint;
mod biguint;
mod errors;

#[pymodule]
fn bigint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<biguint::BigUint>()?;
    m.add_class::<bigsint::BigInt>()?;
    return Ok(());
}
