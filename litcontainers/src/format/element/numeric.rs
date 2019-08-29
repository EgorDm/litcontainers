use super::{Element, Scalar};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use num_traits::{Num, NumCast, Float};
use std::iter::Sum;
use num_complex::Complex;

pub trait NumericElement: Element +
	Add<Output=Self> + AddAssign<Self> +
	Sub<Output=Self> + SubAssign<Self> +
	Mul<Output=Self> + MulAssign<Self> +
	Div<Output=Self> + DivAssign<Self> +
	Rem<Output=Self> + RemAssign<Self> +
	Num + NumCast + Sum + Default
{
	fn from_usize(v: usize) -> Self { Self::from(v).unwrap() }

	fn is_float() -> bool { false }

	fn is_complex() -> bool { false }

	type ScalarType: Scalar;

	fn as_scalar(&self) -> Self::ScalarType;
}

macro_rules! impl_numeric_element (
	($(($Type: ty, $is_float: expr)),* $(,)*) => {$(
		impl NumericElement for $Type {
			fn is_float() -> bool { $is_float }

			type ScalarType = Self;

			fn as_scalar(&self) -> Self::ScalarType { self.clone() }
		}
	)*}
);

impl_numeric_element!(
	(u8, false),
	(i8, false),
	(u16, false),
	(i16, false),
	(u32, false),
	(i32, false),
	(u64, false),
	(i64, false),
	(u128, false),
	(i128, false),
	(f32, true),
	(f64, true),
);

#[allow(non_camel_case_types)]
pub type c32 = Complex<f32>;
#[allow(non_camel_case_types)]
pub type c64 = Complex<f64>;

impl<T: NumericElement + Scalar + Float> NumericElement for Complex<T> {
	type ScalarType = T;

	fn as_scalar(&self) -> Self::ScalarType { self.norm() }
}