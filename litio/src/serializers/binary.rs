use crate::format::*;
use crate::error::*;
use crate::{StorageSerializerLossy, GeneralSerializer, GeneralDeserializer, StorageDeserializerLossy};
use litcontainers::*;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Header {
	element_type: u8,
	element_size: u64,
	rows: u64,
	cols: u64,
	row_stride: u64,
	col_stride: u64,
}

impl Header {
	pub fn element_type(&self) -> Option<ScalarType> {
		bitmask_to_scalar(self.element_type)
	}
}

pub struct BinarySerializer<T, R, C, S>
	where T: Scalar + Serialize, R: Dim, C: Dim, S: Storage<T, R, C>,
{
	_phantoms: PhantomData<(T, R, C, S)>
}

impl<T, R, C, S> StorageSerializerLossy<T, R, C, S> for BinarySerializer<T, R, C, S>
	where T: Scalar + Serialize, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>,
{}

/// Responsible for serializing storage into a binary format
/// Output looks like this:
impl<T, R, C, S> GeneralSerializer<S> for BinarySerializer<T, R, C, S>
	where T: Scalar + Serialize, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>,
{
	/// Serializes storage to binary format into a writer
	fn write<W: std::io::Write>(writer: &mut W, storage: &S) -> IOResult<()> {
		let header = Header {
			element_type: scalar_to_bitmask(T::get_scalar_type()),
			element_size: T::get_scalar_size() as u64,
			rows: storage.row_count() as u64,
			cols: storage.col_count() as u64,
			row_stride: storage.row_stride() as u64,
			col_stride: storage.col_stride() as u64
		};

		let header_bytes = bincode::serialize(&header)?;
		writer.write(header_bytes.as_slice())?;

		let body: Vec<_> = storage.as_iter().cloned().collect();
		let body_bytes = bincode::serialize(&body)?;
		writer.write(&body_bytes)?;
		Ok(())
	}
}

pub struct BinaryDeserializer<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>,
	      for<'de> T: Deserialize<'de>
{
	_phantoms: PhantomData<(T, R, C, S)>
}

impl<T, R, C, S> StorageDeserializerLossy<T, R, C, S> for BinaryDeserializer<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>,
	      for<'de> T: Deserialize<'de>
{}

impl<T, R, C, S> GeneralDeserializer<S> for BinaryDeserializer<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>,
	      for<'de> T: Deserialize<'de>
{
	fn read<RD: std::io::Read>(reader: RD) -> IOResult<S> {
		let mut reader = reader;

		let header: Header = bincode::deserialize_from(&mut reader)?;
		match header.element_type() {
			Some(t) if t == T::get_scalar_type() => {},
			_ => return Err(df_error("Invaid element format!"))
		}

		let rows = R::try_from_usize(header.rows as usize).ok_or(df_error("Invalid row dimension!"))?;
		let cols = C::try_from_usize(header.cols as usize).ok_or(df_error("Invalid col dimension!"))?;
		let row_stride = S::RStride::try_from_usize(header.row_stride as usize).ok_or(df_error("Invalid row stride dimension!"))?;
		let col_stride = S::CStride::try_from_usize(header.col_stride as usize).ok_or(df_error("Invalid col stride dimension!"))?;

		let body: Vec<T> = bincode::deserialize_from(&mut reader)?;
		let ret = S::from_vec(rows, cols, &body);
		if row_stride.value() != ret.row_stride() || col_stride.value() != ret.col_stride() {
			return Err(df_error("Invalid storage strides!"))
		}

		Ok(ret)
	}
}

pub fn write_binary<T, R, C, S, W>(writer: &mut W, data: &S) -> IOResult<()>
	where T: Scalar + Serialize, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>, W: std::io::Write
{
	BinarySerializer::write(writer, data)
}

pub fn read_binary<T, R, C, S, RD>(reader: RD) -> IOResult<S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>, RD: std::io::Read,
	      for<'de> T: Deserialize<'de>
{
	BinaryDeserializer::read(reader)
}

pub fn write_binary_file<T, R, C, S>(path: &Path, data: &S) -> IOResult<()>
	where T: Scalar + Serialize, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>,
{
	crate::file::write::<BinarySerializer<_, _, _, _>, _>(path, data)
}

pub fn read_binary_file<T, R, C, S>(path: &Path) -> IOResult<S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C> + StorageConstructor<T, R, C>,
	      for<'de> T: Deserialize<'de>
{
	crate::file::read::<BinaryDeserializer<_, _, _, _>, _>(path)
}