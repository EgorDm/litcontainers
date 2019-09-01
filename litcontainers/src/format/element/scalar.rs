use super::NumericElement;
use num_traits::float::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScalarType {
	U8   = 1,
	I8   = 2,
	U16  = 3,
	I16  = 4,
	U32  = 5,
	I32  = 6,
	U64  = 7,
	I64  = 8,
	U128 = 9,
	I128 = 10,
	F32  = 11,
	F64  = 12,
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

			fn min_val() -> Self { <$Type>::min_value() }
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