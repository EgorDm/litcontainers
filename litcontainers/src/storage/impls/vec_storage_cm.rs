use crate::format::*;
use crate::storage::{Storage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor, Ownable};
use std::cmp::min;

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	data: Vec<T>,
	size: Size<R, C>
}

impl<T, R, C> VecStorageCM<T, R, C>
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

impl<T, R, C> StorageSize for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type Rows = R;
	type Cols = C;

	fn row_dim(&self) -> R { self.size.row_dim() }

	fn col_dim(&self) -> C { self.size.col_dim() }
}

impl<T, R, C> Strided for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type RowStride = U1;
	type ColStride = R;

	fn row_stride_dim(&self) -> Self::RowStride { U1 }

	fn col_stride_dim(&self) -> Self::ColStride { self.row_dim() }
}

impl<T, R, C> Storage<T> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn as_ptr(&self) -> *const T { self.data.as_ptr() }
}

impl<T, R, C> StorageMut<T> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn as_ptr_mut(&mut self) -> *mut T { self.data.as_mut_ptr() }
}

impl<T, R> DynamicColStorage<T> for VecStorageCM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	fn set_cols(&mut self, count: usize) {
		unsafe {self.resize_element_count(count * self.rows())};
		self.size.cols = Dynamic::from(count);
	}
}

impl<T, C> DynamicRowStorage<T> for VecStorageCM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	fn set_rows(&mut self, count: usize) {
		if count == self.cols() { return; }

		let mut new_data = vec![T::default(); self.cols() * count];
		let copy_size = min(self.cols(), count);
		for ci in 0..self.cols() {
			let to = &mut new_data[ci * count..ci * count + copy_size];
			let from = &self.data[ci * self.rows()..ci * self.rows() + copy_size];
			to.clone_from_slice(from)
		}

		self.data = new_data;
		self.size.rows = Dynamic::from(count);
	}
}

impl<T, R, C> StorageConstructor<T> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn from_value(s: SSize<Self>, value: T) -> Self {
		let len = s.len();
		Self::from_data(s, vec![value; len])
	}
}

impl<T, R, C> Ownable<T> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type OwnedType = Self;

	fn owned(self) -> Self::OwnedType { self }

	fn clone_owned(&self) -> Self::OwnedType {
		Self::from_data(self.size(), self.data.clone())
	}
}