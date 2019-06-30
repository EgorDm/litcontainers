use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use num_traits::NumCast;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScalarType {
    Float,
    Double,
    UInt8,
    Int16,
    Int32,
    Int64,
}

pub trait Scalar:
    Copy + Clone + Debug + Sized + Default +
    Add<Output=Self> + AddAssign<Self> +
    Sub<Output=Self> + SubAssign<Self> +
    Mul<Output=Self> + MulAssign<Self> +
    Div<Output=Self> + DivAssign<Self> +
	NumCast
{
    fn get_scalar_type() -> ScalarType;

    fn get_scalar_size() -> usize {
        size_of::<Self>()
    }

    fn from_usize(v: usize) -> Self { Self::from(v).unwrap() }
}

impl Scalar for f32 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Float
    }
}

impl Scalar for f64 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Double
    }
}

impl Scalar for i32 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Int32
    }
}

impl Scalar for i64 {
    fn get_scalar_type() -> ScalarType { ScalarType::Int64 }
}

impl Scalar for i16 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Int16
    }
}

impl Scalar for u8 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::UInt8
    }
}