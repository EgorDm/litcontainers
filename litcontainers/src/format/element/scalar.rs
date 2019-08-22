use super::NumericElement;
use num_traits::float::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScalarType {
	U8,
	I8,
	U16,
	I16,
	U32,
	I32,
	U64,
	I64,
	U128,
	I128,
	F32,
	F64,
}

pub trait Scalar: NumericElement + PartialOrd {
	fn scalar_type() -> ScalarType;

	fn max_val() -> Self;

	fn min_val() -> Self;
}

macro_rules! impl_scalar (
	($($Type: ty => $TypeEnum: ident),* $(,)*) => {$(
		impl Scalar for $Type {
			fn scalar_type() -> ScalarType { ScalarType::$TypeEnum }

			fn max_val() -> Self { <$Type>::max_value() }

			fn min_val() -> Self { <$Type>::max_value() }
		}
	)*}
);

impl_scalar!(
	u8 => U8,
	i8 => I8,
	u16 => U16,
	i16 => I16,
	u32 => U32,
	i32 => I32,
	u64 => U64,
	i64 => I64,
	u128 => U128,
	i128 => I128,
	f32 => F32,
	f64 => F64
);