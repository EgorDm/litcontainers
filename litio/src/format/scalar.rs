pub use litcontainers::format::*;

pub fn scalar_to_bitmask(t: ScalarType) -> u8 {
	match t {
		ScalarType::Complex(simple_type) => 1 | elementary_scalar_to_bitmask(simple_type),
		_ => {
			let simple_type: Option<ScalarElementaryType> = t.into();
			elementary_scalar_to_bitmask(simple_type.unwrap())
		}
	}
}

pub fn elementary_scalar_to_bitmask(t: ScalarElementaryType) -> u8 {
	match t {
		ScalarElementaryType::Float => 2,
		ScalarElementaryType::Double => 4,
		ScalarElementaryType::UInt8 => 8,
		ScalarElementaryType::Int16 => 16,
		ScalarElementaryType::Int32 => 32,
		ScalarElementaryType::Int64 => 64,
	}
}

pub fn bitmask_to_scalar(m: u8) -> Option<ScalarType> {
	match m {
		m if 1 & m == 1 => bitmask_to_elementary_scalar(m & !1).map(|t| ScalarType::Complex(t)),
		m => bitmask_to_elementary_scalar(m).map(|t| t.into())
	}
}

pub fn bitmask_to_elementary_scalar(m: u8) -> Option<ScalarElementaryType> {
	match m {
		2 => Some(ScalarElementaryType::Float),
		4 => Some(ScalarElementaryType::Double),
		8 => Some(ScalarElementaryType::UInt8),
		16 => Some(ScalarElementaryType::Int16),
		32 => Some(ScalarElementaryType::Int32),
		64 => Some(ScalarElementaryType::Int64),
		_ => None
	}
}