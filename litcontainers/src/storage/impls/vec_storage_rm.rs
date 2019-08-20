use crate::format::*;
use crate::storage::{Storage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor, Ownable};
use std::cmp::min;

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	data: Vec<T>,
	size: Size<R, C>
}

impl<T, R, C> VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	pub fn from_data(size: Size<R, C>, data: Vec<T>) -> Self {
		assert_eq!(size.len(), data.len(), "Data size must match dimensions!");
		Self { data, size }
	}

	unsafe fn resize_element_count(&mut self, size: usize) {
		if self.data.len() > size {
			self.data.resize(size, T::default());
		} else {
			self.data.reserve_exact(size - self.data.len());
			self.data.resize(size, T::default());
		}
	}
}

impl<T, R, C> StorageSize for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type Rows = R;
	type Cols = C;

	fn row_dim(&self) -> Self::Rows { self.size.row_dim() }

	fn col_dim(&self) -> Self::Cols { self.size.col_dim() }
}

impl<T, R, C> Strided for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type RowStride = C;
	type ColStride = U1;

	fn row_stride_dim(&self) -> Self::RowStride { self.col_dim() }

	fn col_stride_dim(&self) -> Self::ColStride { U1 }
}

impl<T, R, C> Storage<T> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn as_ptr(&self) -> *const T { self.data.as_ptr() }
}

impl<T, R, C> StorageMut<T> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn as_ptr_mut(&mut self) -> *mut T { self.data.as_mut_ptr() }
}

impl<T, C> DynamicRowStorage<T> for VecStorageRM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	fn set_row_count(&mut self, count: usize) {
		unsafe {self.resize_element_count(count * self.cols())};
		self.size.rows = Dynamic::from(count);
	}
}

impl<T, R> DynamicColStorage<T> for VecStorageRM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	fn set_col_count(&mut self, count: usize) {
		if count == self.cols() { return; }

		let mut new_data = vec![T::default(); self.rows() * count];
		let copy_size = min(self.cols(), count);
		for ri in 0..self.rows() {
			let to = &mut new_data[ri * count..ri * count + copy_size];
			let from = &self.data[ri * self.cols()..ri * self.cols() + copy_size];
			to.clone_from_slice(from)
		}

		self.data = new_data;
		self.size.cols = Dynamic::from(count);
	}
}

impl<T, R, C> StorageConstructor<T> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn from_value(s: SSize<Self>, value: T) -> Self {
		let len = s.len();
		Self::from_data(s, vec![value; len])
	}
}

impl<T, R, C> Ownable<T> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type OwnedType = Self;

	fn owned(self) -> Self::OwnedType { self }

	fn clone_owned(&self) -> Self::OwnedType {
		Self::from_data(self.size(), self.data.clone())
	}
}