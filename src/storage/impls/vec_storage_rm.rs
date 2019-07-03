use crate::format::*;
use crate::storage::{Storage, SizedStorage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor, Ownable};


#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	data: Vec<T>,
	row_dim: R,
	col_dim: C,
}

impl<T, R, C> VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	unsafe fn resize_element_count(&mut self, size: usize) {
		if self.data.len() > size {
			self.data.resize(size, T::default());
		} else {
			self.data.reserve_exact(size - self.data.len());
			self.data.resize(size, T::default());
		}
	}
}

impl<T, R, C> SizedStorage<R, C> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn row_dim(&self) -> R { self.row_dim }

	fn col_dim(&self) -> C { self.col_dim }
}

impl<T, R, C> Storage<T, R, C> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type RStride = C;
	type CStride = U1;

	fn row_stride_dim(&self) -> Self::RStride { self.col_dim() }

	fn col_stride_dim(&self) -> Self::CStride { U1 }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.as_ptr().offset(i as isize) }
}

impl<T, R, C> StorageMut<T, R, C> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.data.as_mut_ptr().offset(i as isize)
	}
}

impl<T, C> DynamicRowStorage<T, C> for VecStorageRM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	fn set_row_count(&mut self, count: usize) {
		unsafe {self.resize_element_count(count * self.col_count())};
		self.row_dim = Dynamic::from(count);
	}
}

impl<T, R> DynamicColStorage<T, R> for VecStorageRM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	fn set_col_count(&mut self, count: usize) {
		let mut new_data = vec![T::default(); self.row_count() * count];

		for ri in 0..self.row_count() {
			let to = &mut new_data[ri * count..ri * count + self.col_count()];
			let from = &self.data[ri * self.col_count()..ri * self.col_count() + self.col_count()];
			to.clone_from_slice(from)
		}

		self.data = new_data;
		self.col_dim = Dynamic::from(count);
	}
}

impl<T, R, C> StorageConstructor<T, R, C> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn from_value(rows: R, cols: C, value: T) -> Self {
		Self::new(rows, cols, vec![value; rows.value() * cols.value()])
	}
}

impl<T, R, C> VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	pub fn new(rows: R, cols: C, data: Vec<T>) -> Self {
		assert_eq!(rows.value() * cols.value(), data.len(), "Data size must match dimensions!");
		Self { data, row_dim: rows, col_dim: cols, }
	}
}

impl<T, R, C> Ownable<T, R, C> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type OwnedType = Self;

	fn owned(self) -> Self::OwnedType { self }

	fn clone_owned(&self) -> Self::OwnedType {
		Self::new(self.row_dim(), self.col_dim(), self.data.clone())
	}
}
