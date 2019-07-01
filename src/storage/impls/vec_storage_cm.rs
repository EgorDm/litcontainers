use crate::format::*;
use crate::storage::{Storage, SizedStorage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor};

pub type VecStorageCM<T, R, C> = VecStorageBaseCM<T, R, U1, C, R>;

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageBaseCM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	data: Vec<T>,
	row_dim: R,
	col_dim: C,
	row_stride: RS,
	col_stride: CS,
}


impl<T, R, RS, C, CS> VecStorageBaseCM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn resize_element_count(&mut self, size: usize) {
		if self.data.len() > size {
			self.data.set_len(size);
			self.data.shrink_to_fit();
		} else {
			self.data.reserve_exact(size - self.data.len());
			self.data.set_len(size);
		}
	}
}

impl<T, R, RS, C, CS> SizedStorage<R, C> for VecStorageBaseCM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn row_dim(&self) -> R { self.row_dim }

	fn col_dim(&self) -> C { self.col_dim }
}

impl<T, R, RS, C, CS> Storage<T, R, C> for VecStorageBaseCM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type RStride = RS;
	type CStride = CS;

	fn row_stride_dim(&self) -> Self::RStride { self.row_stride }

	fn col_stride_dim(&self) -> Self::CStride { self.col_stride }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.as_ptr().offset(i as isize) }
}

impl<T, R, RS, C, CS> StorageMut<T, R, C> for VecStorageBaseCM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.data.as_mut_ptr().offset(i as isize)
	}
}

impl<T, R> DynamicColStorage<T, R> for VecStorageCM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	unsafe fn set_col_count(&mut self, count: usize) {
		self.resize_element_count(count * self.row_count());
		self.col_dim = Dynamic::from(count);
	}
}

impl<T, C> DynamicRowStorage<T, C> for VecStorageCM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	unsafe fn set_row_count(&mut self, count: usize) {
		self.row_dim = Dynamic::from(count);
		self.col_stride = Dynamic::from(count);
		let mut new_data = vec![T::default(); self.col_count() * count];

		for ci in 0..self.col_count() {
			let to = &mut new_data[ci * count..ci * count + self.row_count()];
			let from = &self.data[ci * self.row_count()..ci * self.row_count() + self.row_count()];
			to.clone_from_slice(from)
		}
	}
}

impl<T, R> StorageConstructor<T, R, Dynamic> for VecStorageCM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	fn from_value(rows: R, cols: Dynamic, value: T) -> Self {
		VecStorageCM {
			data: vec![value; rows.value() * cols.value()],
			row_dim: rows,
			col_dim: cols,
			row_stride: U1,
			col_stride: rows
		}
	}
}
