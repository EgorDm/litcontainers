use crate::format::*;
use crate::storage::*;
use crate::{Slice, SliceBase, SliceRange, SliceMut};

pub trait Sliceable<T: Scalar>: Storage<T> {
	#[inline]
	fn slice_rows<RR: SliceRange<Self::Rows>>(&self, range: RR)
		-> Slice<T, RR::Size, Self::RowStride, Self::Cols, Self::ColStride>
	{
		assert!(self.rows() <= range.end(), "Range is out of bounds!");
		unsafe {
			PtrStorage::new(
				self.as_row_ptr(range.begin()),
				Size::new(range.size(), self.col_dim()),
				Strides::new(self.row_stride_dim(), self.col_stride_dim())
			).into()
		}
	}

	#[inline]
	fn slice_cols<CR: SliceRange<Self::Cols>>(&self, range: CR)
		-> Slice<T, Self::Rows, Self::RowStride, CR::Size, Self::ColStride>
	{
		assert!(self.cols() <= range.end(), "Range is out of bounds!");
		unsafe {
			PtrStorage::new(
				self.as_col_ptr(range.begin()),
				Size::new(self.row_dim(), range.size()),
				Strides::new(self.row_stride_dim(), self.col_stride_dim())
			).into()
		}
	}

	#[inline]
	fn slice<RR: SliceRange<Self::Rows>, CR: SliceRange<Self::Cols>>(&self, range_rows: RR, range_cols: CR)
		-> Slice<T, RR::Size, Self::RowStride, CR::Size, Self::ColStride>
	{
		assert!(self.cols() <= range_cols.end() && self.rows() <= range_rows.end(), "Range is out of bounds!");
		unsafe {
			PtrStorage::new(
				self.get_ptr(range_rows.begin(), range_cols.begin()),
				Size::new(range_rows.size(), range_cols.size()),
				Strides::new(self.row_stride_dim(), self.col_stride_dim())
			).into()
		}
	}
}

pub trait SliceableMut<T: Scalar>: StorageMut<T> {
	#[inline]
	fn slice_rows_mut<RR: SliceRange<Self::Rows>>(&mut self, range: RR)
		-> SliceMut<T, RR::Size, Self::RowStride, Self::Cols, Self::ColStride>
	{
		assert!(self.rows() <= range.end(), "Range is out of bounds!");
		unsafe {
			PtrStorageMut::new(
				self.as_row_ptr_mut(range.begin()),
				Size::new(range.size(), self.col_dim()),
				Strides::new(self.row_stride_dim(), self.col_stride_dim())
			).into()
		}
	}

	#[inline]
	fn slice_cols_mut<CR: SliceRange<Self::Cols>>(&mut self, range: CR)
		-> SliceMut<T, Self::Rows, Self::RowStride, CR::Size, Self::ColStride>
	{
		assert!(self.cols() <= range.end(), "Range is out of bounds!");
		unsafe {
			PtrStorageMut::new(
				self.as_col_ptr_mut(range.begin()),
				Size::new(self.row_dim(), range.size()),
				Strides::new(self.row_stride_dim(), self.col_stride_dim())
			).into()
		}
	}

	#[inline]
	fn slice_mut<RR: SliceRange<Self::Rows>, CR: SliceRange<Self::Cols>>(&mut self, range_rows: RR, range_cols: CR)
		-> SliceMut<T, RR::Size, Self::RowStride, CR::Size, Self::ColStride>
	{
		assert!(self.cols() <= range_cols.end() && self.rows() <= range_rows.end(), "Range is out of bounds!");
		unsafe {
			PtrStorageMut::new(
				self.get_ptr_mut(range_rows.begin(), range_cols.begin()),
				Size::new(range_rows.size(), range_cols.size()),
				Strides::new(self.row_stride_dim(), self.col_stride_dim())
			).into()
		}
	}
}