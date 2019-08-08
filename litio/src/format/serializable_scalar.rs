use litcontainers::{Scalar, ElementaryScalar};
use num_complex::Complex;
use num_traits::Float;
use serde::{Serializer, Serialize, Deserialize, Deserializer};

pub trait SerializableScalar: Scalar {
	fn serialize_scalar<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

pub trait DeserializableScalar: Scalar {
	fn deserialize_scalar<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

macro_rules! impl_serializable_scalar {
	( $( $Type:ty ),* ) => {
		$(
		impl SerializableScalar for $Type {
			fn serialize_scalar<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { <Self as Serialize>::serialize(self, serializer) }

		}

		impl DeserializableScalar for $Type {
			fn deserialize_scalar<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { <Self as Deserialize>::deserialize(deserializer) }
		}
		)*
	}
}

impl_serializable_scalar!(f32, f64, i16, i32, i64, u8);

#[derive(Serialize, Deserialize)]
#[serde(remote = "Complex")]
struct ComplexDef<T> {
	pub re: T,
	pub im: T,
}


impl<T: Serialize + ElementaryScalar + Float> SerializableScalar for Complex<T> {
	fn serialize_scalar<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { ComplexDef::serialize(self, serializer) }
}

impl<T: for<'de> Deserialize<'de> + ElementaryScalar + Float> DeserializableScalar for Complex<T> {
	fn deserialize_scalar<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		ComplexDef::deserialize(deserializer)
	}
}

pub struct ScalarSerializer<T: SerializableScalar>(T);

impl<T: SerializableScalar> ScalarSerializer<T> {
	pub fn new(data: T) -> Self { Self(data) }
}

impl<T: SerializableScalar> Serialize for ScalarSerializer<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		self.0.serialize_scalar(serializer)
	}
}

pub struct ScalarDeserializer<T>(T);

impl<T> ScalarDeserializer<T>
	where T: DeserializableScalar
{
	pub fn data(self) -> T { self.0 }
}

impl<'de, T: DeserializableScalar> Deserialize<'de> for ScalarDeserializer<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where D: Deserializer<'de> {
		T::deserialize_scalar(deserializer).map(|data| Self(data))
	}
}
