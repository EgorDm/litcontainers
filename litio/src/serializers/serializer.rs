use litcontainers::*;
use crate::IOResult;

pub trait StorageSerializerLossy<T, R, C, S>: GeneralSerializer<S>
	where T: Scalar + serde::Serialize, R: Dim, C: Dim, S: Storage<T, R, C>,
{}

pub trait GeneralSerializer<T>
{
	fn write<W: std::io::Write>(writer: &mut W, storage: &T) -> IOResult<()>;
}

pub trait StorageDeserializerLossy<T, R, C, S>: GeneralDeserializer<S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>,
	      for<'de> T: serde::Deserialize<'de>
{}

pub trait GeneralDeserializer<T>
{
	fn read<'de, RD: std::io::Read>(reader: RD) -> IOResult<T>;
}