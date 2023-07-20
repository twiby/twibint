use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXorAssign, Div, Mul, Rem, Shl,
    ShlAssign, Shr, ShrAssign, Sub,
};

pub trait Digit:
    Copy
    + std::fmt::Debug
    + std::fmt::Binary
    + std::fmt::LowerHex
    + std::fmt::UpperHex
    + std::hash::Hash
    + std::cmp::Ord
    + From<bool>
    + TryInto<u16>
    + TryInto<u32>
    + TryInto<u64>
    + TryFrom<Self::Double>
    + PartialEq
    + AddAssign
    + Add<Output = Self>
    + Sub<Output = Self>
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + ShrAssign<usize>
    + ShlAssign<usize>
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
{
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const NB_BITS: usize;
    type Double: DoubleDigit<Single = Self>;
    fn to_double(self) -> Self::Double;
    fn overflowing_sub(self, other: Self) -> (Self, bool);
    fn leading_zeros(self) -> u32;
    fn decomposition_from_u32(n: u32) -> Vec<Self>;
    fn decomposition_from_u64(n: u64) -> Vec<Self>;
}
pub trait DoubleDigit:
    Copy
    + PartialEq
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<usize, Output = Self>
    + BitOr<Output = Self>
{
    const ZERO: Self;
    type Single: Digit<Double = Self>;
    fn truncate_upper(self) -> Self::Single;
    fn truncate_lower(self) -> Self::Single;
    fn split(self) -> (Self::Single, Self::Single) {
        (self.truncate_upper(), self.truncate_lower())
    }
}

impl Digit for u32 {
    const ZERO: u32 = 0u32;
    const ONE: u32 = 1u32;
    const MAX: u32 = u32::MAX;
    const NB_BITS: usize = 32;
    type Double = u64;
    fn to_double(self) -> u64 {
        self as u64
    }
    fn overflowing_sub(self, other: Self) -> (Self, bool) {
        self.overflowing_sub(other)
    }
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }
    fn decomposition_from_u32(n: u32) -> Vec<Self> {
        vec![n]
    }
    fn decomposition_from_u64(n: u64) -> Vec<Self> {
        vec![n as u32, (n >> 32) as u32]
    }
}
impl DoubleDigit for u64 {
    const ZERO: u64 = 0u64;
    type Single = u32;
    fn truncate_upper(self) -> u32 {
        self as u32
    }
    fn truncate_lower(self) -> u32 {
        (self >> 32) as u32
    }
}

impl Digit for u64 {
    const ZERO: u64 = 0u64;
    const ONE: u64 = 1u64;
    const MAX: u64 = u64::MAX;
    const NB_BITS: usize = 64;
    type Double = u128;
    fn to_double(self) -> u128 {
        self as u128
    }
    fn overflowing_sub(self, other: Self) -> (Self, bool) {
        self.overflowing_sub(other)
    }
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }
    fn decomposition_from_u32(n: u32) -> Vec<Self> {
        vec![n as u64]
    }
    fn decomposition_from_u64(n: u64) -> Vec<Self> {
        vec![n]
    }
}
impl DoubleDigit for u128 {
    const ZERO: u128 = 0u128;
    type Single = u64;
    fn truncate_upper(self) -> u64 {
        self as u64
    }
    fn truncate_lower(self) -> u64 {
        (self >> 64) as u64
    }
}
