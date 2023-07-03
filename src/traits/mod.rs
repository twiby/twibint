pub type DivisionResult<T> = Result<T, crate::errors::DivisionByZero>;

pub trait RemDiv<T> {
    type DivOutput;
    type RemOutput;
    fn rem_div(&self, other: &T) -> DivisionResult<(Self::DivOutput, Self::RemOutput)>;
    fn div(&self, other: &T) -> DivisionResult<Self::DivOutput> {
        self.rem_div(other).map(|ret| ret.0)
    }
    fn rem(&self, other: &T) -> DivisionResult<Self::RemOutput> {
        self.rem_div(other).map(|ret| ret.1)
    }
}

pub trait TrueDiv<T> {
    fn truediv(&self, other: &T) -> DivisionResult<f64>;
}

pub trait Pow {
    fn pow(&self, other: usize) -> Self;
}
