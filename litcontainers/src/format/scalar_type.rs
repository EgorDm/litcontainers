use std::fmt::{Debug, Formatter, Error};
use std::mem::size_of;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use num_traits::{NumCast, Num, cast::cast, Float};
use num_complex::Complex;
use std::iter::Sum;

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
	UInt32
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
			ScalarElementaryType::UInt32 => ScalarType::UInt32,
		}
	}
}

impl Into<Option<ScalarElementaryType>> for ScalarType {
	fn into(self) -> Option<ScalarElementaryType> {
		match self {
			ScalarType::Float => Some(ScalarElementaryType::Float),
			ScalarType::Double => Some(ScalarElementaryType::Double),
			ScalarType::UInt8 => Some(ScalarElementaryType::UInt8),
			ScalarType::Int16 => Some(ScalarElementaryType::Int16),
			ScalarType::Int32 => Some(ScalarElementaryType::Int32),
			ScalarType::Int64 => Some(ScalarElementaryType::Int64),
			ScalarType::UInt32 => Some(ScalarElementaryType::UInt32),
			_ => None
		}
	}
}

// TODO: Refector Scalar -> Element and ElementaryType -> Scalar

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScalarType {
	Float,
	Double,
	UInt8,
	Int16,
	Int32,
	Int64,
	UInt32,
	Complex(ScalarElementaryType),
}

pub trait ElementaryScalar: Scalar + PartialOrd
{
	fn get_elementary_scalar_type() -> ScalarElementaryType;

	fn max_val() -> Self;

	fn min_val() -> Self;
}

pub trait Scalar:
	Copy + Clone + Debug + Sized + Default + Send + Sync +
	Add<Output=Self> + AddAssign<Self> +
	Sub<Output=Self> + SubAssign<Self> +
	Mul<Output=Self> + MulAssign<Self> +
	Div<Output=Self> + DivAssign<Self> +
	Rem<Output=Self> + RemAssign<Self> +
	Num + NumCast + Sum
{
	fn get_scalar_type() -> ScalarType;

	fn get_scalar_size() -> usize {
		size_of::<Self>()
	}

	fn from_usize(v: usize) -> Self { Self::from(v).unwrap() }

	fn fmt_num(&self, f: &mut Formatter, precision: usize, sci: bool) -> Result<(), Error>;

	fn is_float() -> bool { false }

	fn is_complex() -> bool { false }

	type ElementaryType: ElementaryScalar;

	fn to_elementary(&self) -> Self::ElementaryType;
}

impl ElementaryScalar for f32 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::Float
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl Scalar for f32 {
	fn get_scalar_type() -> ScalarType {
		ScalarType::Float
	}

	fn fmt_num(&self, f: &mut Formatter, precision: usize, sci: bool) -> Result<(), Error> {
		if sci {
			write!(f, "{:+.*e}", precision, self)
		} else {
			write!(f, "{:.*}", precision, self)
		}
	}

	fn is_float() -> bool { true }

	type ElementaryType = f32;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}

impl ElementaryScalar for f64 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::Double
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl Scalar for f64 {
	fn get_scalar_type() -> ScalarType {
		ScalarType::Double
	}

	fn fmt_num(&self, f: &mut Formatter, precision: usize, sci: bool) -> Result<(), Error> {
		if sci {
			write!(f, "{:+.*e}", precision, self)
		} else {
			write!(f, "{:.*}", precision, self)
		}
	}

	fn is_float() -> bool { true }

	type ElementaryType = f64;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}

impl ElementaryScalar for i32 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::Int32
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl Scalar for i32 {
	fn get_scalar_type() -> ScalarType {
		ScalarType::Int32
	}

	fn fmt_num(&self, f: &mut Formatter, precision: usize, sci: bool) -> Result<(), Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f32)
		} else {
			write!(f, "{:.*}", precision, *self as f32)
		}
	}

	type ElementaryType = i32;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}

impl ElementaryScalar for i64 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::Int64
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl Scalar for i64 {
	fn get_scalar_type() -> ScalarType { ScalarType::Int64 }

	fn fmt_num(&self, f: &mut Formatter, precision: usize, sci: bool) -> Result<(), Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{:.*}", precision, *self as f64)
		}
	}

	type ElementaryType = i64;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}

impl ElementaryScalar for i16 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::Int16
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl Scalar for i16 {
	fn get_scalar_type() -> ScalarType {
		ScalarType::Int16
	}

	fn fmt_num(&self, f: &mut Formatter, precision: usize, sci: bool) -> Result<(), Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f32)
		} else {
			write!(f, "{:.*}", precision, *self as f32)
		}
	}

	type ElementaryType = i16;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}

impl ElementaryScalar for u8 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::UInt8
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl ElementaryScalar for u32 {
	fn get_elementary_scalar_type() -> ScalarElementaryType {
		ScalarElementaryType::UInt32
	}

	fn max_val() -> Self { Self::max_value() }

	fn min_val() -> Self { Self::min_value ()}
}

impl Scalar for u32 {
	fn get_scalar_type() -> ScalarType {
		ScalarType::UInt32
	}

	fn fmt_num(&self, f: &mut Formatter, _precision: usize, _sci: bool) -> Result<(), Error> {
		write!(f, "{}", self)
	}

	type ElementaryType = u32;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}


impl Scalar for u8 {
	fn get_scalar_type() -> ScalarType {
		ScalarType::UInt8
	}

	fn fmt_num(&self, f: &mut Formatter, _precision: usize, _sci: bool) -> Result<(), Error> {
		write!(f, "{}", self)
	}

	type ElementaryType = u8;

	fn to_elementary(&self) -> Self::ElementaryType { *self }
}

impl<T: ElementaryScalar + Float> Scalar for Complex<T> {
	fn get_scalar_type() -> ScalarType {
		ScalarType::Complex(T::get_elementary_scalar_type())
	}

	fn fmt_num(&self, f: &mut Formatter, precision: usize, _sci: bool) -> Result<(), Error> {
		let re: f64 = cast(self.re).unwrap();
		let im: f64 = cast(self.im).unwrap();

		write!(f, "({: >11},{: >11})", format!("{:+.*e}", precision, re), format!("{:+.*e}", precision, im))
	}

	fn is_complex() -> bool { true }

	type ElementaryType = T;

	fn to_elementary(&self) -> Self::ElementaryType { self.norm() }
}

impl<T: ElementaryScalar + Float> crate::Sqrt for Complex<T> {
	type Output = Self;

	fn sqrt(self) -> Self::Output { Complex::sqrt(&self) }
}

impl<T: ElementaryScalar + Float> crate::Sqrt for T {
	type Output = Self;

	fn sqrt(self) -> Self::Output { <T as Float>::sqrt(self) }
}