pub use litcontainers::format::*;

pub fn element_to_byte(t: ElementType) -> u8 {
	match t {
		ElementType::Scalar(s) => scalar_to_byte(s),
		ElementType::Complex(s) => scalar_to_byte(s) | 128,
		ElementType::Bool => 64,
	}
}

pub fn scalar_to_byte(t: ScalarType) -> u8 { t as u8 }

pub fn element_from_byte(m: u8) -> Option<ElementType> {
	match m {
		m if 64 & m == 64 => Some(ElementType::Bool),
		m if 128 & m == 128 => scalar_from_byte(m & !128).map(|v| ElementType::Complex(v)),
		m => scalar_from_byte(m).map(|v| ElementType::Scalar(v)),
	}
}

pub fn scalar_from_byte(m: u8) -> Option<ScalarType> {
	match m {
		1   => Some(ScalarType::U8),
		2   => Some(ScalarType::I8),
		3   => Some(ScalarType::U16),
		4   => Some(ScalarType::I16),
		5   => Some(ScalarType::U32),
		6   => Some(ScalarType::I32),
		7   => Some(ScalarType::U64),
		8   => Some(ScalarType::I64),
		9   => Some(ScalarType::U128),
		10  => Some(ScalarType::I128),
		11  => Some(ScalarType::F32),
		12  => Some(ScalarType::F64),
		_ => None
	}
}