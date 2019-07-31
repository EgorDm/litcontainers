use crate::format::*;
use crate::storage::*;
use crate::{Slice, SliceMut};

pub trait Transposable<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn t(&self) -> Slice<T, C, Self::CStride, R, Self::RStride> {
		Slice::new(unsafe {
			PtrStorage::new(
				self.get_index_ptr_unchecked(0),
				self.col_dim(),
				self.row_dim(),
				self.col_stride_dim(),
				self.row_stride_dim(),
			)
		})
	}

	fn transmute_dims<RO, CO, RSO, CSO>(&self, row_dim: RO, col_dim: CO, row_stride: RSO, col_stride: CSO)
		-> Slice<T, RO, RSO, CO, CSO>
		where RO: Dim, CO: Dim, RSO: Dim, CSO: Dim
	{
		let new_size = (row_dim.value() - 1) * row_stride.value() + (col_dim.value() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		Slice::new(unsafe {
			PtrStorage::new(
				self.get_index_ptr_unchecked(0),
				row_dim,
				col_dim,
				row_stride,
				col_stride,
			)
		})
	}

	fn transmute_stride_dims<RSO, CSO>(&self, row_stride: RSO, col_stride: CSO)
		-> Slice<T, R, RSO, C, CSO>
		where RSO: Dim, CSO: Dim
	{
		let new_size = (self.row_count() - 1) * row_stride.value() + (self.col_count() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		Slice::new(unsafe {
			PtrStorage::new(
				self.get_index_ptr_unchecked(0),
				self.row_dim(),
				self.col_dim(),
				row_stride,
				col_stride,
			)
		})
	}
}

pub trait TransposableMut<T, R, C>: StorageMut<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn t_mut(&mut self) -> SliceMut<T, C, Self::CStride, R, Self::RStride> {
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.get_index_mut_ptr_unchecked(0),
				self.col_dim(),
				self.row_dim(),
				self.col_stride_dim(),
				self.row_stride_dim(),
			)
		})
	}

	fn transmute_dims_mut<RO, CO, RSO, CSO>(&mut self, row_dim: RO, col_dim: CO, row_stride: RSO, col_stride: CSO)
		-> SliceMut<T, RO, RSO, CO, CSO>
		where RO: Dim, CO: Dim, RSO: Dim, CSO: Dim
	{
		let new_size = (row_dim.value() - 1) * row_stride.value() + (col_dim.value() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.get_index_mut_ptr_unchecked(0),
				row_dim,
				col_dim,
				row_stride,
				col_stride,
			)
		})
	}

	fn transmute_stride_dims_mut<RSO, CSO>(&mut self, row_stride: RSO, col_stride: CSO)
		-> SliceMut<T, R, RSO, C, CSO>
		where RSO: Dim, CSO: Dim
	{
		let new_size = (self.row_count() - 1) * row_stride.value() + (self.col_count() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.get_index_mut_ptr_unchecked(0),
				self.row_dim(),
				self.col_dim(),
				row_stride,
				col_stride,
			)
		})
	}
}

pub trait TransposableInplace<'a, T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn transmute_dims_inplace<RO, CO, RSO, CSO>(self, row_dim: RO, col_dim: CO, row_stride: RSO, col_stride: CSO)
		-> Slice<'a, T, RO, RSO, CO, CSO>
		where RO: Dim, CO: Dim, RSO: Dim, CSO: Dim;

	fn transmute_stride_dims_inplace<RSO, CSO>(self, row_stride: RSO, col_stride: CSO)
		-> Slice<'a, T, R, RSO, C, CSO>
		where RSO: Dim, CSO: Dim;
}

pub trait TransposableInplaceMut<'a, T, R, C>: StorageMut<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn transmute_dims_inplace<RO, CO, RSO, CSO>(self, row_dim: RO, col_dim: CO, row_stride: RSO, col_stride: CSO)
		-> SliceMut<'a, T, RO, RSO, CO, CSO>
		where RO: Dim, CO: Dim, RSO: Dim, CSO: Dim;

	fn transmute_stride_dims_inplace<RSO, CSO>(self, row_stride: RSO, col_stride: CSO)
		-> SliceMut<'a, T, R, RSO, C, CSO>
		where RSO: Dim, CSO: Dim;
}


// TODO: if this gives problems -> move into storage?

impl<T, R, C, S> Transposable<T, R, C> for S
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{}

impl<T, R, C, S> TransposableMut<T, R, C> for S
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{}
