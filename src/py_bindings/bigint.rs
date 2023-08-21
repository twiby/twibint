use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp::*;

use crate::traits::{Pow, RemDiv, TrueDiv};
use crate::{BigInt, BigUint};

#[pyclass]
#[derive(Clone)]
pub struct PyBigInt(pub BigInt<u64>);

impl AsRef<BigInt<u64>> for PyBigInt {
    fn as_ref(&self) -> &BigInt<u64> {
        &self.0
    }
}

#[pymethods]
impl PyBigInt {
    #[new]
    /// Python constructor for BigInt, implicitely using the TryFrom<PyAny> implementation. \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __init__(n: &PyAny) -> PyResult<Self> {
        Ok(Self::try_from(n)?)
    }

    /// Python binding of the `abs` operation
    pub fn __abs__(&self) -> Self {
        let mut ret = self.clone();
        ret.0.sign = true;
        ret
    }

    /// Python binding to the unary neg operation.
    pub fn __neg__(&self) -> Self {
        Self(-self.as_ref())
    }

    /// Python binding to convert to a float (returns a double
    /// precision floating point number)
    pub fn __float__(&self) -> f64 {
        self.as_ref().into()
    }

    /// Python binding to convert to a int
    pub fn __int__(&self, py: Python<'_>) -> PyResult<PyObject> {
        if self.0.sign {
            self.0.uint.__int__(py)
        } else {
            Ok(self.0.uint.__int__(py)?.call_method0(py, "__neg__")?)
        }
    }

    /// Python binding for the `+` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() + Self::try_from(other)?.as_ref()))
    }
    /// Python binding for the `+=` operation, specifically binded here because
    /// the crate implements an actual add assign operation, more efficient than
    /// add + clone. \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __iadd__(&mut self, other: &PyAny) -> PyResult<()> {
        self.0 += Self::try_from(other)?.as_ref();
        Ok(())
    }
    /// Python binding for the reverse `+` operation, allowing performing an addition
    /// of the form (Python int) + (BigInt). \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __radd__(&self, other: &PyAny) -> PyResult<Self> {
        self.__add__(other)
    }

    /// Python binding for the `-` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() - Self::try_from(other)?.as_ref()))
    }
    /// Python binding for the `-=` operation, specifically binded here because
    /// the crate implements an actual sub assign operation, more efficient than
    /// sub + clone. \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __isub__(&mut self, other: &PyAny) -> PyResult<()> {
        self.0 -= Self::try_from(other)?.as_ref();
        Ok(())
    }
    /// Python binding for the reverse `-` operation, allowing performing an subtraction
    /// of the form (Python int) - (BigInt). \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __rsub__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(Self::try_from(other)?.as_ref() - self.as_ref()))
    }

    /// Python binding to the `*` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() * Self::try_from(other)?.as_ref()))
    }
    /// Python binding for the reverse `*` operation, allowing performing an multiplication
    /// of the form (Python int) * (BigInt). \
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __rmul__(&self, other: &PyAny) -> PyResult<Self> {
        self.__mul__(other)
    }

    /// Python binding to the `//` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    pub fn __floordiv__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() / Self::try_from(other)?.as_ref()))
    }
    /// Python binding to the reverse `//` operation, allowing performing a floor division
    /// of the form (Python int) // (BigInt). \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    pub fn __rfloordiv__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(Self::try_from(other)?.as_ref() / self.as_ref()))
    }

    /// Python binding to the `%` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    pub fn __mod__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() % Self::try_from(other)?.as_ref()))
    }
    /// Python binding to the reverse `%` operation, allowing performing a floor division
    /// of the form (Python int) % (BigInt). \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    pub fn __rmod__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(Self::try_from(other)?.as_ref() % self.as_ref()))
    }

    /// Python binding to the `divmod` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    pub fn __divmod__(&self, other: &PyAny) -> PyResult<(Self, Self)> {
        let (q, r) = BigInt::<u64>::rem_div(self.as_ref(), Self::try_from(other)?.as_ref())?;
        Ok((Self(q), Self(r)))
    }
    /// Python binding to the reverse `divmod` operation. \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    pub fn __rdivmod__(&self, other: &PyAny) -> PyResult<(Self, Self)> {
        let (q, r) = BigInt::<u64>::rem_div(Self::try_from(other)?.as_ref(), self.as_ref())?;
        Ok((Self(q), Self(r)))
    }

    /// Python binding to the `/` operation (returns a double precision float).
    /// Only implemented for systems with a little endian float format. \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    #[cfg(target_endian = "little")]
    pub fn __truediv__(&self, other: &PyAny) -> PyResult<f64> {
        Ok(BigInt::<u64>::truediv(
            self.as_ref(),
            Self::try_from(other)?.as_ref(),
        )?)
    }
    /// Python binding to the reverse `/` operation (returns a double precision float).
    /// Only implemented for systems with a little endian float format. \
    /// This will raise an error if the operand is not compatible with a BigInt,
    /// or in the case of a divison by zero.
    #[cfg(target_endian = "little")]
    pub fn __rtruediv__(&self, other: &PyAny) -> PyResult<f64> {
        Ok(BigInt::<u64>::truediv(
            Self::try_from(other)?.as_ref(),
            self.as_ref(),
        )?)
    }

    /// Python binding to the `**` operation. \
    /// This will raise an error when called with the modulus argument.
    pub fn __pow__(&self, other: usize, modulus: Option<usize>) -> PyResult<Self> {
        if matches!(modulus, Some(_)) {
            Err(PyErr::new::<PyValueError, _>(
                "Modulus argument of pow function not supported",
            ))
        } else {
            Ok(Self(BigInt::<u64>::pow(self.as_ref(), other)))
        }
    }

    /// Python binding to the `<<` operation
    pub fn __lshift__(&self, n: usize) -> Self {
        Self(self.as_ref() << n)
    }
    /// Python binding to the `>>` operation
    pub fn __rshift__(&self, n: usize) -> Self {
        Self(self.as_ref() >> n)
    }
    /// Python binding to the `<<=` operation
    pub fn __ilshift__(&mut self, n: usize) {
        self.0 <<= n;
    }
    /// Python binding to the `>>=` operation
    pub fn __irshift__(&mut self, n: usize) {
        self.0 >>= n;
    }

    /// Python binding to the `&` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __and__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() & Self::try_from(other)?.as_ref()))
    }
    /// Python binding to the `&=` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __iand__(&mut self, other: &PyAny) -> PyResult<()> {
        self.0 &= Self::try_from(other)?.as_ref();
        Ok(())
    }
    /// Python binding to the reverse `&` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __rand__(&self, other: &PyAny) -> PyResult<Self> {
        self.__and__(other)
    }

    /// Python binding to the `|` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __or__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() | Self::try_from(other)?.as_ref()))
    }
    /// Python binding to the `|=` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __ior__(&mut self, other: &PyAny) -> PyResult<()> {
        self.0 |= Self::try_from(other)?.as_ref();
        Ok(())
    }
    /// Python binding to the reverse `|` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __ror__(&self, other: &PyAny) -> PyResult<Self> {
        self.__or__(other)
    }

    /// Python binding to the `^` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __xor__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(Self(self.as_ref() ^ Self::try_from(other)?.as_ref()))
    }
    /// Python binding to the `^=` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __ixor__(&mut self, other: &PyAny) -> PyResult<()> {
        self.0 ^= Self::try_from(other)?.as_ref();
        Ok(())
    }
    /// Python binding to the reverse `^` operation.
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __rxor__(&self, other: &PyAny) -> PyResult<Self> {
        self.__xor__(other)
    }

    /// Python binding to the invert `~` operation.
    pub fn __invert__(&self) -> Self {
        Self(!self.as_ref())
    }

    /// Python binding to all the comparators: `==`, `!=`, `<`, `<=`, `>`, and `>=`
    /// This will raise an error if the operand is not compatible with a BigInt.
    pub fn __richcmp__(&self, other: &PyAny, cmp: pyo3::basic::CompareOp) -> PyResult<bool> {
        let int = Self::try_from(other)?;
        Ok(match cmp {
            Lt => self.as_ref() < int.as_ref(),
            Le => self.as_ref() <= int.as_ref(),
            Eq => self.as_ref() == int.as_ref(),
            Ne => self.as_ref() != int.as_ref(),
            Gt => self.as_ref() > int.as_ref(),
            Ge => self.as_ref() >= int.as_ref(),
        })
    }

    /// Python binding to the `bool` function
    pub fn __bool__(&self) -> bool {
        self.0.uint != BigUint::default()
    }

    /// Python binding to the `str` function
    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.as_ref().to_string())
    }
    /// Python binding to the `repr` function
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.as_ref()))
    }

    /// Python binding to the `len` function. This is for debug purposes,
    /// as this returns internal information that aren't supposed to be useful
    /// for the users of this module.
    pub fn __len__(&self) -> usize {
        self.0.uint.val.len()
    }
}
