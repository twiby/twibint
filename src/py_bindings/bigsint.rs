use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp::*;

pub use crate::{BigInt, BigUint};

// TODO: typical python services: __int__, bitwise, bitshifts

#[pymethods]
impl BigInt {
    #[new]
    pub fn __init__(n: &PyAny) -> PyResult<Self> {
        Ok(Self::try_from(n)?)
    }
    #[cfg(target_endian = "little")]
    #[staticmethod]
    pub fn from_f32(n: f32) -> PyResult<Self> {
        Ok(n.try_into()?)
    }
    #[cfg(target_endian = "little")]
    #[staticmethod]
    pub fn from_f64(n: f64) -> PyResult<Self> {
        Ok(n.try_into()?)
    }

    pub fn __abs__(&self) -> Self {
        Self {
            uint: self.uint.clone(),
            sign: true,
        }
    }
    pub fn __neg__(&self) -> Self {
        -self
    }

    pub fn __float__(&self) -> f64 {
        self.into()
    }

    pub fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self + &Self::try_from(other)?)
    }
    pub fn __radd__(&self, other: &PyAny) -> PyResult<Self> {
        self.__add__(other)
    }
    pub fn __iadd__(&mut self, other: &PyAny) -> PyResult<()> {
        *self += &Self::try_from(other)?;
        Ok(())
    }

    pub fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self - &Self::try_from(other)?)
    }
    pub fn __rsub__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(&Self::try_from(other)? - self)
    }
    pub fn __isub__(&mut self, other: &PyAny) -> PyResult<()> {
        *self -= &Self::try_from(other)?;
        Ok(())
    }

    pub fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self * &Self::try_from(other)?)
    }
    pub fn __rmul__(&self, other: &PyAny) -> PyResult<Self> {
        self.__mul__(other)
    }

    pub fn __floordiv__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self / &Self::try_from(other)?)
    }
    pub fn __rfloordiv__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(&Self::try_from(other)? / self)
    }

    pub fn __mod__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self % &Self::try_from(other)?)
    }
    pub fn __rmod__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(&Self::try_from(other)? % self)
    }

    pub fn __divmod__(&self, other: &PyAny) -> PyResult<(Self, Self)> {
        Ok(
            <BigInt as crate::biguint::ops::divrem::RemDiv<BigInt>>::rem_div(
                self,
                &Self::try_from(other)?,
            )?,
        )
    }
    pub fn __rdivmod__(&self, other: &PyAny) -> PyResult<(Self, Self)> {
        Ok(
            <BigInt as crate::biguint::ops::divrem::RemDiv<BigInt>>::rem_div(
                &Self::try_from(other)?,
                self,
            )?,
        )
    }
    #[cfg(target_endian = "little")]
    pub fn __truediv__(&self, other: &PyAny) -> PyResult<f64> {
        Ok(
            <BigInt as crate::biguint::ops::truediv::TrueDiv<BigInt>>::truediv(
                self,
                &Self::try_from(other)?,
            )?,
        )
    }
    #[cfg(target_endian = "little")]
    pub fn __rtruediv__(&self, other: &PyAny) -> PyResult<f64> {
        Ok(
            <BigInt as crate::biguint::ops::truediv::TrueDiv<BigInt>>::truediv(
                &Self::try_from(other)?,
                self,
            )?,
        )
    }
    pub fn __pow__(&self, other: usize, modulus: Option<usize>) -> PyResult<Self> {
        if matches!(modulus, Some(_)) {
            Err(PyErr::new::<PyValueError, _>(
                "Modulus argument of pow function not supported",
            ))
        } else {
            Ok(self.pow(other))
        }
    }

    // pub fn __lshift__(&self, n: usize) -> Self {
    //     self << n
    // }
    // pub fn __rshift__(&self, n: usize) -> Self {
    //     self >> n
    // }
    // pub fn __ilshift__(&mut self, n: usize) {
    //     *self <<= n;
    // }
    // pub fn __irshift__(&mut self, n: usize) {
    //     *self >>= n;
    // }

    // pub fn __and__(&self, other: &Self) -> Self {
    //     self & other
    // }
    // pub fn __or__(&self, other: &Self) -> Self {
    //     self | other
    // }
    // pub fn __xor__(&self, other: &Self) -> Self {
    //     self ^ other
    // }

    pub fn __richcmp__(&self, other: &PyAny, cmp: pyo3::basic::CompareOp) -> PyResult<bool> {
        let int = Self::try_from(other)?;
        Ok(match cmp {
            Lt => self < &int,
            Le => self <= &int,
            Eq => self == &int,
            Ne => self != &int,
            Gt => self > &int,
            Ge => self >= &int,
        })
    }

    pub fn __bool__(&self) -> bool {
        self.uint != BigUint::default()
    }

    pub fn __str__(&self) -> PyResult<String> {
        return Ok(self.to_string());
    }
    pub fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("{:?}", self));
    }
}
