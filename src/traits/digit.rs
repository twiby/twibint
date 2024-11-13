use std::any::TypeId;
use std::ops::RemAssign;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXorAssign, Div, Mul, Rem, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait ToPtr {
    fn to_ptr<T: 'static>(&self) -> Option<*const T>;
    fn to_mut_ptr<T: 'static>(&mut self) -> Option<*mut T>;
    fn from_ptr<T: 'static>(ptr: *const T) -> Option<*const Self>;
}

impl<T: Digit> ToPtr for T {
    fn to_ptr<T2: 'static>(&self) -> Option<*const T2> {
        if TypeId::of::<T>() == TypeId::of::<T2>() {
            Some((self as *const T).cast())
        } else {
            None
        }
    }
    fn to_mut_ptr<T2: 'static>(&mut self) -> Option<*mut T2> {
        if TypeId::of::<T>() == TypeId::of::<T2>() {
            Some((self as *mut T).cast())
        } else {
            None
        }
    }
    fn from_ptr<T2: 'static>(ptr: *const T2) -> Option<*const Self> {
        if TypeId::of::<T>() == TypeId::of::<T2>() {
            Some(ptr as *const T)
        } else {
            None
        }
    }
}

impl<T: Digit> ToPtr for [T] {
    fn to_ptr<T2: 'static>(&self) -> Option<*const T2> {
        if TypeId::of::<T>() == TypeId::of::<T2>() {
            Some(self.as_ptr().cast())
        } else {
            None
        }
    }
    fn to_mut_ptr<T2: 'static>(&mut self) -> Option<*mut T2> {
        if TypeId::of::<T>() == TypeId::of::<T2>() {
            Some(self.as_mut_ptr().cast())
        } else {
            None
        }
    }
    fn from_ptr<T2: 'static>(_: *const T2) -> Option<*const Self> {
        unimplemented!();
    }
}

pub trait Digit:
    'static
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
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
    + SubAssign
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
    + ToPtr
where
    [Self]: ToPtr,
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const NB_BITS: usize;
    type Double: DoubleDigit<Single = Self>;
    type Signed: SignedDigit<Unsigned = Self>;
    fn to_double(self) -> Self::Double;
    fn from_bool(b: bool) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
    fn overflowing_sub(self, other: Self) -> (Self, bool);
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn decomposition_from_u32(n: u32) -> Vec<Self>;
    fn decomposition_from_u64(n: u64) -> Vec<Self>;
    fn write_bytes(self, buff: &mut [u8]);
    fn read_bytes(buff: &[u8]) -> Self;
}
pub trait DoubleDigit:
    Copy
    + std::fmt::Debug
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + RemAssign
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + BitOr<Output = Self>
    + BitOrAssign
{
    const ZERO: Self;
    const MAX: Self;
    const HALF_MAX: Self;
    type Single: Digit<Double = Self>;
    fn truncate_upper(self) -> Self::Single;
    fn truncate_lower(self) -> Self::Single;
    /// returns (lsb, msb)
    #[inline]
    fn split(self) -> (Self::Single, Self::Single) {
        (self.truncate_upper(), self.truncate_lower())
    }
    #[cfg(test)]
    fn pack(d: &[Self::Single]) -> Self;
}
pub trait SignedDigit: Copy {
    type Unsigned: Digit<Signed = Self>;
    fn abs(self) -> Self::Unsigned;
    fn is_positive(self) -> bool;
}

impl Digit for u32 {
    const ZERO: u32 = 0u32;
    const ONE: u32 = 1u32;
    const TWO: u32 = 2u32;
    const MAX: u32 = u32::MAX;
    const NB_BITS: usize = 32;
    type Double = u64;
    type Signed = i32;
    #[inline]
    fn to_double(self) -> u64 {
        self as u64
    }
    #[inline]
    fn overflowing_sub(self, other: Self) -> (Self, bool) {
        self.overflowing_sub(other)
    }
    #[inline]
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    #[inline]
    fn from_bool(b: bool) -> Self {
        b as Self
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }
    #[inline]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }
    #[inline]
    fn decomposition_from_u32(n: u32) -> Vec<Self> {
        vec![n]
    }
    #[inline]
    fn decomposition_from_u64(n: u64) -> Vec<Self> {
        vec![n as u32, (n >> 32) as u32]
    }
    #[inline]
    fn write_bytes(self, buff: &mut [u8]) {
        for (i, b) in self.to_le_bytes().into_iter().enumerate() {
            buff[i] = b;
        }
    }
    #[inline]
    fn read_bytes(buff: &[u8]) -> Self {
        Self::from_le_bytes(buff.try_into().unwrap())
    }
}
impl DoubleDigit for u64 {
    const ZERO: u64 = 0u64;
    const MAX: u64 = u64::MAX;
    const HALF_MAX: u64 = u64::MAX >> 32;
    type Single = u32;
    #[inline]
    fn truncate_upper(self) -> u32 {
        self as u32
    }
    #[inline]
    fn truncate_lower(self) -> u32 {
        (self >> 32) as u32
    }
    #[cfg(test)]
    #[inline]
    fn pack(d: &[Self::Single]) -> Self {
        debug_assert_eq!(d.len(), 2);
        (d[0] as Self) + ((d[1] as Self) << 32)
    }
}
impl SignedDigit for i32 {
    type Unsigned = u32;
    #[inline]
    fn abs(self) -> u32 {
        self.abs().try_into().unwrap()
    }
    #[inline]
    fn is_positive(self) -> bool {
        i32::is_positive(self)
    }
}

impl Digit for u64 {
    const ZERO: u64 = 0u64;
    const ONE: u64 = 1u64;
    const TWO: u64 = 2u64;
    const MAX: u64 = u64::MAX;
    const NB_BITS: usize = 64;
    type Double = u128;
    type Signed = i64;
    #[inline]
    fn to_double(self) -> u128 {
        self as u128
    }
    #[inline]
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    #[inline]
    fn from_bool(b: bool) -> Self {
        b as Self
    }
    #[inline]
    fn overflowing_sub(self, other: Self) -> (Self, bool) {
        self.overflowing_sub(other)
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }
    #[inline]
    fn trailing_zeros(self) -> u32 {
        self.trailing_zeros()
    }
    #[inline]
    fn decomposition_from_u32(n: u32) -> Vec<Self> {
        vec![n as u64]
    }
    #[inline]
    fn decomposition_from_u64(n: u64) -> Vec<Self> {
        vec![n]
    }
    #[inline]
    fn write_bytes(self, buff: &mut [u8]) {
        for (i, b) in self.to_le_bytes().into_iter().enumerate() {
            buff[i] = b;
        }
    }
    #[inline]
    fn read_bytes(buff: &[u8]) -> Self {
        Self::from_le_bytes(buff.try_into().unwrap())
    }
}
impl DoubleDigit for u128 {
    const ZERO: u128 = 0u128;
    const MAX: u128 = u128::MAX;
    const HALF_MAX: u128 = u128::MAX >> 64;
    type Single = u64;
    #[inline]
    fn truncate_upper(self) -> u64 {
        self as u64
    }
    #[inline]
    fn truncate_lower(self) -> u64 {
        (self >> 64) as u64
    }
    #[cfg(test)]
    #[inline]
    fn pack(d: &[Self::Single]) -> Self {
        debug_assert_eq!(d.len(), 2);
        (d[0] as Self) + ((d[1] as Self) << 64)
    }
}
impl SignedDigit for i64 {
    type Unsigned = u64;
    #[inline]
    fn abs(self) -> u64 {
        self.abs().try_into().unwrap()
    }
    #[inline]
    fn is_positive(self) -> bool {
        i64::is_positive(self)
    }
}
