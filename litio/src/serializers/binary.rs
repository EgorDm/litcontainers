use crate::format::*;
use crate::error::*;
use crate::{StorageSerializerLossy, GeneralSerializer, GeneralDeserializer, StorageDeserializerLossy};
use litcontainers::*;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use std::path::Path;

// TODO: can be improved alot by serialilzing certain types like size, stride. But works just fine for now

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
	pub fn element_type(&self) -> Option<ElementType> {
		element_from_byte(self.element_type)
	}
}

pub struct BinarySerializer<T, S>
	where T: NumericElement + SerializableScalar, S: Storage<T>,
{
	_phantoms: PhantomData<(T, S)>
}

impl<T, S> StorageSerializerLossy<T, S> for BinarySerializer<T, S>
	where T: NumericElement + SerializableScalar, S: Storage<T>
{}

/// Responsible for serializing storage into a binary format
/// Output looks like this:
impl<T, S> GeneralSerializer<S> for BinarySerializer<T, S>
	where T: NumericElement + SerializableScalar, S: Storage<T>
{
	/// Serializes storage to binary format into a writer
	fn write<W: std::io::Write>(writer: &mut W, storage: &S) -> IOResult<()> {
		let header = Header {
			element_type: element_to_byte(T::element_type()),
			element_size: T::byte_size() as u64,
			rows: storage.rows() as u64,
			cols: storage.cols() as u64,
			row_stride: storage.row_stride() as u64,
			col_stride: storage.col_stride() as u64
		};

		let header_bytes = bincode::serialize(&header)?;
		writer.write(header_bytes.as_slice())?;

		let body: Vec<_> = storage.as_iter().cloned().map(|v| ScalarSerializer::new(v)).collect();
		let body_bytes = bincode::serialize(&body)?;
		writer.write(&body_bytes)?;
		Ok(())
	}
}

pub struct BinaryDeserializer<T, S>
	where T: NumericElement + DeserializableScalar, S: Storage<T>,
{
	_phantoms: PhantomData<(T, S)>
}

impl<T, S> StorageDeserializerLossy<T, S> for BinaryDeserializer<T, S>
	where T: NumericElement + DeserializableScalar, S: Storage<T> + StorageConstructor<T>,
{}

impl<T, S> GeneralDeserializer<S> for BinaryDeserializer<T, S>
	where T: NumericElement + DeserializableScalar, S: Storage<T> + StorageConstructor<T>,
{
	fn read<RD: std::io::Read>(reader: RD) -> IOResult<S> {
		let mut reader = reader;

		let header: Header = bincode::deserialize_from(&mut reader)?;
		let test = T::element_type();
		let test2 = header.element_type();
		match header.element_type() {
			Some(t) if t == T::element_type() => {},
			_ => return Err(df_error("Invaid element format!"))
		}

		let rows = S::Rows::try_from_usize(header.rows as usize).ok_or(df_error("Invalid row dimension!"))?;
		let cols = S::Cols::try_from_usize(header.cols as usize).ok_or(df_error("Invalid col dimension!"))?;
		let row_stride = S::RowStride::try_from_usize(header.row_stride as usize).ok_or(df_error("Invalid row stride dimension!"))?;
		let col_stride = S::ColStride::try_from_usize(header.col_stride as usize).ok_or(df_error("Invalid col stride dimension!"))?;

		let body: Vec<ScalarDeserializer<T>> = bincode::deserialize_from(&mut reader)?;
		let body: Vec<_> = body.into_iter().map(|v| v.data()).collect();
		let ret = S::from_vec(Size::new(rows, cols), &body);
		if row_stride.value() != ret.row_stride() || col_stride.value() != ret.col_stride() {
			return Err(df_error("Invalid storage strides!"))
		}

		Ok(ret)
	}
}

pub fn write_binary<T, S, W>(writer: &mut W, data: &S) -> IOResult<()>
	where T: NumericElement + SerializableScalar, S: Storage<T>, W: std::io::Write
{
	BinarySerializer::write(writer, data)
}

pub fn read_binary<T, S, RD>(reader: RD) -> IOResult<S>
	where T: NumericElement + DeserializableScalar, S: Storage<T> + StorageConstructor<T>, RD: std::io::Read,
{
	BinaryDeserializer::read(reader)
}

pub fn write_binary_file<T, S>(path: &Path, data: &S) -> IOResult<()>
	where T: NumericElement + SerializableScalar, S: Storage<T>,
{
	crate::file::write::<BinarySerializer<_, _>, _>(path, data)
}

pub fn read_binary_file<T, S>(path: &Path) -> IOResult<S>
	where T: NumericElement + DeserializableScalar, S: Storage<T> + StorageConstructor<T>,
{
	crate::file::read::<BinaryDeserializer<_, _>, _>(path)
}