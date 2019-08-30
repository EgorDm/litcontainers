use litcontainers::*;
use crate::{IOResult, SerializableScalar, DeserializableScalar};

pub trait StorageSerializerLossy<T, S>: GeneralSerializer<S>
	where T: Scalar + SerializableScalar, S: Storage<T>,
{}

pub trait GeneralSerializer<T>
{
	fn write<W: std::io::Write>(writer: &mut W, storage: &T) -> IOResult<()>;
}

pub trait StorageDeserializerLossy<T, S>: GeneralDeserializer<S>
	where T: Scalar + DeserializableScalar, S: Storage<T>,
{}

pub trait GeneralDeserializer<T>
{
	fn read<'de, RD: std::io::Read>(reader: RD) -> IOResult<T>;
}