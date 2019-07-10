use std::fmt::{Debug, Display};
use std::mem::size_of;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use num_traits::{NumCast, Num};
use num_complex::Complex;

#[allow(non_camel_case_types)]
pub type c32 = Complex<f32>;
#[allow(non_camel_case_types)]
pub type c64 = Complex<f64>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScalarElementaryType {
    Float,
    Double,
    UInt8,
    Int16,
    Int32,
    Int64,
}

impl Into<ScalarType> for ScalarElementaryType {
    fn into(self) -> ScalarType {
        match self {
            ScalarElementaryType::Float => ScalarType::Float,
            ScalarElementaryType::Double => ScalarType::Double,
            ScalarElementaryType::UInt8 => ScalarType::UInt8,
            ScalarElementaryType::Int16 => ScalarType::Int16,
            ScalarElementaryType::Int32 => ScalarType::Int32,
            ScalarElementaryType::Int64 => ScalarType::Int64,
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScalarType {
    Float,
    Double,
    UInt8,
    Int16,
    Int32,
    Int64,
    Complex(ScalarElementaryType),
}

pub trait ElementaryScalar: Scalar
{
    fn get_elementary_scalar_type() -> ScalarElementaryType;
}

pub trait Scalar:
    Copy + Clone + Debug + Sized + Default +
    Add<Output=Self> + AddAssign<Self> +
    Sub<Output=Self> + SubAssign<Self> +
    Mul<Output=Self> + MulAssign<Self> +
    Div<Output=Self> + DivAssign<Self> +
    Rem<Output=Self> + RemAssign<Self> +
	Num + NumCast
{
    fn get_scalar_type() -> ScalarType;

    fn get_scalar_size() -> usize {
        size_of::<Self>()
    }

    fn from_usize(v: usize) -> Self { Self::from(v).unwrap() }
}

impl ElementaryScalar for f32 {
    fn get_elementary_scalar_type() -> ScalarElementaryType {
        ScalarElementaryType::Float
    }
}

impl Scalar for f32 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Float
    }
}

impl ElementaryScalar for f64 {
    fn get_elementary_scalar_type() -> ScalarElementaryType {
        ScalarElementaryType::Double
    }
}

impl Scalar for f64 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Double
    }
}

impl ElementaryScalar for i32 {
    fn get_elementary_scalar_type() -> ScalarElementaryType {
        ScalarElementaryType::Int32
    }
}

impl Scalar for i32 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Int32
    }
}

impl ElementaryScalar for i64 {
    fn get_elementary_scalar_type() -> ScalarElementaryType {
        ScalarElementaryType::Int64
    }
}

impl Scalar for i64 {
    fn get_scalar_type() -> ScalarType { ScalarType::Int64 }
}

impl ElementaryScalar for i16 {
    fn get_elementary_scalar_type() -> ScalarElementaryType {
        ScalarElementaryType::Int16
    }
}

impl Scalar for i16 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Int16
    }
}

impl ElementaryScalar for u8 {
    fn get_elementary_scalar_type() -> ScalarElementaryType {
        ScalarElementaryType::UInt8
    }
}

impl Scalar for u8 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::UInt8
    }
}

/*impl Scalar for c32 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Complex(ScalarType::Float)
    }
}

impl Scalar for c64 {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Complex(ScalarType::Double)
    }
}*/

impl<T: ElementaryScalar> Scalar for Complex<T> {
    fn get_scalar_type() -> ScalarType {
        ScalarType::Complex(T::get_elementary_scalar_type())
    }
}