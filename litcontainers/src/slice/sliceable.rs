use crate::format::*;
use crate::storage::{Storage, PtrStorage, StorageMut, PtrMutStorage};
use crate::slice::{SliceRange, Slice, SliceMut};

pub trait Sliceable<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn slice_rows<'b: 'c, 'c, RR: SliceRange<R>>(&'b self, range: RR) -> Slice<'c, T, RR::Size, Self::RStride, C, Self::CStride> {
		assert!(range.end() <= self.row_count(), "Slice is out of bounds!");
		//TODO: cound check
		Slice::new(unsafe {
			PtrStorage::new(
				self.as_row_ptr(range.begin()),
				range.size(),
				self.col_dim(),
				self.row_stride_dim(),
				self.col_stride_dim(),
			)
		})
	}

	#[inline]
	fn slice_cols<'b: 'c, 'c, CR: SliceRange<C>>(&'b self, range: CR) -> Slice<'c, T, R, Self::RStride, CR::Size, Self::CStride> {
		assert!(range.end() <= self.col_count(), "Slice is out of bounds!");
		Slice::new(unsafe {
			PtrStorage::new(
				self.as_col_ptr(range.begin()),
				self.row_dim(),
				range.size(),
				self.row_stride_dim(),
				self.col_stride_dim(),
			)
		})
	}

	#[inline]
	fn slice<'b: 'c, 'c, RR: SliceRange<R>, CR: SliceRange<C>>(&'b self, range_rows: RR, range_cols: CR)
		-> Slice<'c, T, RR::Size, Self::RStride, CR::Size, Self::CStride>
	{
		assert!(range_cols.end() <= self.col_count() && range_rows.end() <= self.row_count(), "Slice is out of bounds!");
		Slice::new(unsafe {
			PtrStorage::new(
				self.get_index_ptr_unchecked(self.index(range_rows.begin(), range_cols.begin())),
				range_rows.size(),
				range_cols.size(),
				self.row_stride_dim(),
				self.col_stride_dim(),
			)
		})
	}
}


pub trait SliceableMut<T, R, C>: StorageMut<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn slice_rows_mut<'b: 'c, 'c, RR: SliceRange<R>>(&'b mut self, range: RR) -> SliceMut<'c, T, RR::Size, Self::RStride, C, Self::CStride> {
		assert!(range.end() <= self.row_count(), "Slice is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.as_row_mut_ptr(range.begin()),
				range.size(),
				self.col_dim(),
				self.row_stride_dim(),
				self.col_stride_dim(),
			)
		})
	}

	#[inline]
	fn slice_cols_mut<'b: 'c, 'c, CC: SliceRange<C>>(&'b mut self, range: CC) -> SliceMut<'c, T, R, Self::RStride, CC::Size, Self::CStride> {
		assert!(range.end() <= self.col_count(), "Slice is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.as_col_mut_ptr(range.begin()),
				self.row_dim(),
				range.size(),
				self.row_stride_dim(),
				self.col_stride_dim(),
			)
		})
	}

	#[inline]
	fn slice_mut<'b: 'c, 'c, RR: SliceRange<R>, CR: SliceRange<C>>(&'b mut self, range_rows: RR, range_cols: CR)
		-> SliceMut<'c, T, RR::Size, Self::RStride, CR::Size, Self::CStride>
	{
		assert!(range_cols.end() <= self.col_count() && range_rows.end() <= self.row_count(), "Slice is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.get_index_mut_ptr_unchecked(self.index(range_rows.begin(), range_cols.begin())),
				range_rows.size(),
				range_cols.size(),
				self.row_stride_dim(),
				self.col_stride_dim(),
			)
		})
	}
}