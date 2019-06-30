use crate::format::*;
use crate::storage::{Storage, SizedStorage, StorageMut};

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageBase<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	data: Vec<T>,
	row_dim: R,
	col_dim: C,
	row_stride: RS,
	col_stride: CS,
}

impl<T, R, RS, C, CS> SizedStorage<R, C> for VecStorageBase<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn row_dim(&self) -> R { self.row_dim }

	fn col_dim(&self) -> C { self.col_dim }
}

impl<T, R, RS, C, CS> Storage<T, R, C> for VecStorageBase<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type RStride = RS;
	type CStride = CS;

	fn row_stride_dim(&self) -> Self::RStride { self.row_stride }

	fn col_stride_dim(&self) -> Self::CStride { self.col_stride }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.as_ptr().offset(i as isize) }
}

impl<T, R, RS, C, CS> StorageMut<T, R, C> for VecStorageBase<T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.data.as_mut_ptr().offset(i as isize)
	}
}

