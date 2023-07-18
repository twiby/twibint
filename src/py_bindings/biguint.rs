use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp::*;

use crate::BigUint;

#[pymethods]
impl BigUint {
    #[new]
    /// Python constructor for BigUint, implicitely using the TryFrom<PyAny> implementation. \
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __init__(n: &PyAny) -> PyResult<Self> {
        Ok(Self::try_from(n)?)
    }

    /// Python binding of the `abs` operation
    pub fn __abs__(&self) -> Self {
        self.clone()
    }

    /// Python binding to convert to a float (returns a double
    /// precision floating point number)
    pub fn __float__(&self) -> f64 {
        self.into()
    }

    /// Python binding to convert to a int
    // TODO: not efficient at all, fail between 10000 and 100000 digits
    pub fn __int__(&self, py: Python<'_>) -> PyResult<PyObject> {
        // Lowest bits
        let mut py_obj = self.val[0].to_object(py);

        // Move to highest bits, shifting and adding each time
        for (i, val) in self.val.iter().enumerate().skip(1) {
            let lhs = val
                .to_object(py)
                .call_method1(py, "__lshift__", (32 * i,))?;
            py_obj = py_obj.call_method1(py, "__or__", (lhs,))?;
        }
        Ok(py_obj)
    }

    /// Python binding for the `+` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self + &Self::try_from(other)?)
    }
    /// Python binding for the `+=` operation, specifically binded here because
    /// the crate implements an actual add assign operation, more efficient than
    /// add + clone. \
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __iadd__(&mut self, other: &PyAny) -> PyResult<()> {
        *self += &Self::try_from(other)?;
        Ok(())
    }
    /// Python binding for the reverse `+` operation, allowing performing an addition
    /// of the form (Python int) + (BigUint). \
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __radd__(&self, other: &PyAny) -> PyResult<Self> {
        self.__add__(other)
    }

    /// Python binding for the `-` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint, or
    /// if the subtraction will underflow.
    pub fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self - &Self::try_from(other)?)
    }
    /// Python binding for the `-=` operation, specifically binded here because
    /// the crate implements an actual sub assign operation, more efficient than
    /// sub + clone. \
    /// This will raise an error if the operand is not compatible with a BigUint, or
    /// if the subtraction will underflow.
    pub fn __isub__(&mut self, other: &PyAny) -> PyResult<()> {
        *self -= &Self::try_from(other)?;
        Ok(())
    }
    /// Python binding for the reverse `-` operation, allowing performing an subtraction
    /// of the form (Python int) - (BigUint). \
    /// This will raise an error if the operand is not compatible with a BigUint, or
    /// if the subtraction will underflow.
    pub fn __rsub__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(&Self::try_from(other)? - self)
    }

    /// Python binding to the `*` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self * &Self::try_from(other)?)
    }
    /// Python binding for the reverse `*` operation, allowing performing an multiplication
    /// of the form (Python int) * (BigUint). \
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __rmul__(&self, other: &PyAny) -> PyResult<Self> {
        self.__mul__(other)
    }

    /// Python binding to the `//` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    pub fn __floordiv__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self / &Self::try_from(other)?)
    }
    /// Python binding to the reverse `//` operation, allowing performing a floor division
    /// of the form (Python int) // (BigUint). \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    pub fn __rfloordiv__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(&Self::try_from(other)? / self)
    }

    /// Python binding to the `%` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    pub fn __mod__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self % &Self::try_from(other)?)
    }
    /// Python binding to the reverse `%` operation, allowing performing a floor division
    /// of the form (Python int) % (BigUint). \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    pub fn __rmod__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(&Self::try_from(other)? % self)
    }

    /// Python binding to the `divmod` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    pub fn __divmod__(&self, other: &PyAny) -> PyResult<(Self, Self)> {
        Ok(<BigUint as crate::traits::RemDiv<BigUint>>::rem_div(
            self,
            &Self::try_from(other)?,
        )?)
    }
    /// Python binding to the reverse `divmod` operation. \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    pub fn __rdivmod__(&self, other: &PyAny) -> PyResult<(Self, Self)> {
        Ok(<BigUint as crate::traits::RemDiv<BigUint>>::rem_div(
            &Self::try_from(other)?,
            self,
        )?)
    }

    /// Python binding to the `/` operation (returns a double precision float).
    /// Only implemented for systems with a little endian float format. \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    #[cfg(target_endian = "little")]
    pub fn __truediv__(&self, other: &PyAny) -> PyResult<f64> {
        Ok(<BigUint as crate::traits::TrueDiv<BigUint>>::truediv(
            self,
            &Self::try_from(other)?,
        )?)
    }
    /// Python binding to the reverse `/` operation (returns a double precision float).
    /// Only implemented for systems with a little endian float format. \
    /// This will raise an error if the operand is not compatible with a BigUint,
    /// or in the case of a divison by zero.
    #[cfg(target_endian = "little")]
    pub fn __rtruediv__(&self, other: &PyAny) -> PyResult<f64> {
        Ok(<BigUint as crate::traits::TrueDiv<BigUint>>::truediv(
            &Self::try_from(other)?,
            self,
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
            Ok(<BigUint as crate::traits::Pow>::pow(self, other))
        }
    }

    /// Python binding to the `<<` operation
    pub fn __lshift__(&self, n: usize) -> Self {
        self << n
    }
    /// Python binding to the `>>` operation
    pub fn __rshift__(&self, n: usize) -> Self {
        self >> n
    }
    /// Python binding to the `<<=` operation
    pub fn __ilshift__(&mut self, n: usize) {
        *self <<= n;
    }
    /// Python binding to the `>>=` operation
    pub fn __irshift__(&mut self, n: usize) {
        *self >>= n;
    }

    /// Python binding to the `&` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __and__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self & &Self::try_from(other)?)
    }
    /// Python binding to the `&=` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __iand__(&mut self, other: &PyAny) -> PyResult<()> {
        *self &= &Self::try_from(other)?;
        Ok(())
    }
    /// Python binding to the reverse `&` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __rand__(&self, other: &PyAny) -> PyResult<Self> {
        self.__and__(other)
    }

    /// Python binding to the `|` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __or__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self | &Self::try_from(other)?)
    }
    /// Python binding to the `|=` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __ior__(&mut self, other: &PyAny) -> PyResult<()> {
        *self |= &Self::try_from(other)?;
        Ok(())
    }
    /// Python binding to the reverse `|` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __ror__(&self, other: &PyAny) -> PyResult<Self> {
        self.__or__(other)
    }

    /// Python binding to the `^` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __xor__(&self, other: &PyAny) -> PyResult<Self> {
        Ok(self ^ &Self::try_from(other)?)
    }
    /// Python binding to the `^=` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __ixor__(&mut self, other: &PyAny) -> PyResult<()> {
        *self ^= &Self::try_from(other)?;
        Ok(())
    }
    /// Python binding to the reverse `^` operation.
    /// This will raise an error if the operand is not compatible with a BigUint.
    pub fn __rxor__(&self, other: &PyAny) -> PyResult<Self> {
        self.__xor__(other)
    }

    /// Python binding to all the comparators: `==`, `!=`, `<`, `<=`, `>`, and `>=`
    /// This will raise an error if the operand is not compatible with a BigUint.
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

    /// Python binding to the `bool` function
    pub fn __bool__(&self) -> bool {
        self != &BigUint::default()
    }

    /// Python binding to the `str` function
    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
    /// Python binding to the `repr` function
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    /// Python binding to the `len` function. This is for debug purposes,
    /// as this returns internal information that aren't supposed to be useful
    /// for the users of this module.
    pub fn __len__(&self) -> usize {
        self.val.len()
    }
}
