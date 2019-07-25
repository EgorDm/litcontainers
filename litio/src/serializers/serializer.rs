use litcontainers::*;
use crate::IOResult;

pub trait StorageSerializerLossy<T, R, C, S>: GeneralSerializer<S>
	where T: Scalar + serde::Serialize, R: Dim, C: Dim, S: Storage<T, R, C>,
	      for<'de> T: serde::Deserialize<'de>
{}

pub trait GeneralSerializer<T>
{
	fn write<W: std::io::Write>(writer: &mut W, storage: &T) -> IOResult<()>;

	fn read<'de, RD: std::io::Read>(reader: RD) -> IOResult<T>;
}