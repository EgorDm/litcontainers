use crate::format::*;
use crate::storage::{Storage, SizedStorage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor};


pub type VecStorageRM<T, R, C> = VecStorageBaseRM<T, R, C, C, U1>;

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageBaseRM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	data: Vec<T>,
	row_dim: R,
	col_dim: C,
	row_stride: RS,
	col_stride: CS,
}

impl<T, R, RS, C, CS> VecStorageBaseRM<T, R, RS, C, CS>
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

impl<T, R, RS, C, CS> SizedStorage<R, C> for VecStorageBaseRM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn row_dim(&self) -> R { self.row_dim }

	fn col_dim(&self) -> C { self.col_dim }
}

impl<T, R, RS, C, CS> Storage<T, R, C> for VecStorageBaseRM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type RStride = RS;
	type CStride = CS;

	fn row_stride_dim(&self) -> Self::RStride { self.row_stride }

	fn col_stride_dim(&self) -> Self::CStride { self.col_stride }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.as_ptr().offset(i as isize) }
}

impl<T, R, RS, C, CS> StorageMut<T, R, C> for VecStorageBaseRM<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.data.as_mut_ptr().offset(i as isize)
	}
}

impl<T, C> DynamicRowStorage<T, C> for VecStorageRM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	unsafe fn set_row_count(&mut self, count: usize) {
		self.resize_element_count(count * self.col_count());
		self.row_dim = Dynamic::from(count);
	}
}

impl<T, R> DynamicColStorage<T, R> for VecStorageRM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	unsafe fn set_col_count(&mut self, count: usize) {
		self.col_dim = Dynamic::from(count);
		self.row_stride = Dynamic::from(count);
		let mut new_data = vec![T::default(); self.row_count() * count];

		for ri in 0..self.row_count() {
			let to = &mut new_data[ri * count..ri * count + self.col_count()];
			let from = &self.data[ri * self.col_count()..ri * self.col_count() + self.col_count()];
			to.clone_from_slice(from)
		}
	}
}

impl<T, R, C> StorageConstructor<T, R, C> for VecStorageRM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn from_value(rows: R, cols: C, value: T) -> Self {
		VecStorageRM {
			data: vec![value; rows.value() * cols.value()],
			row_dim: rows,
			col_dim: cols,
			row_stride: cols,
			col_stride: U1
		}
	}
}