use crate::format::*;
use crate::storage::{Storage, SizedStorage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor, Ownable};
use std::cmp::min;
use std::ops::{Index, IndexMut};

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	data: Vec<T>,
	row_dim: R,
	col_dim: C,
}

impl<T, R, C> VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	pub fn new(rows: R, cols: C, data: Vec<T>) -> Self {
		assert_eq!(rows.value() * cols.value(), data.len(), "Data size must match dimensions!");
		Self { data, row_dim: rows, col_dim: cols }
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

impl<T, R, C> SizedStorage<R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn row_dim(&self) -> R { self.row_dim }

	fn col_dim(&self) -> C { self.col_dim }
}

impl<T, R, C> Storage<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type RStride = U1;
	type CStride = R;

	fn row_stride_dim(&self) -> Self::RStride { U1 }

	fn col_stride_dim(&self) -> Self::CStride { self.row_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.as_ptr().offset(i as isize) }
}

impl<T, R, C> StorageMut<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.data.as_mut_ptr().offset(i as isize)
	}
}

impl<T, R> DynamicColStorage<T, R> for VecStorageCM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	fn set_col_count(&mut self, count: usize) {
		unsafe {self.resize_element_count(count * self.row_count())};
		self.col_dim = Dynamic::from(count);
	}
}

impl<T, C> DynamicRowStorage<T, C> for VecStorageCM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	fn set_row_count(&mut self, count: usize) {
		if count == self.col_count() { return; }

		let mut new_data = vec![T::default(); self.col_count() * count];
		let copy_size = min(self.col_count(), count);
		for ci in 0..self.col_count() {
			let to = &mut new_data[ci * count..ci * count + copy_size];
			let from = &self.data[ci * self.row_count()..ci * self.row_count() + copy_size];
			to.clone_from_slice(from)
		}

		self.data = new_data;
		self.row_dim = Dynamic::from(count);
	}
}

impl<T, R, C> StorageConstructor<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn from_value(rows: R, cols: C, value: T) -> Self {
		Self::new(rows, cols, vec![value; rows.value() * cols.value()])
	}
}

impl<T, R, C> Ownable<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type OwnedType = Self;

	fn owned(self) -> Self::OwnedType { self }

	fn clone_owned(&self) -> Self::OwnedType {
		Self::new(self.row_dim(), self.col_dim(), self.data.clone())
	}
}

impl<T, R, C> Index<usize> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		assert!(index < self.size());
		unsafe { &*self.get_index_ptr_unchecked(index) }
	}
}

impl<T, R, C> IndexMut<usize> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}
