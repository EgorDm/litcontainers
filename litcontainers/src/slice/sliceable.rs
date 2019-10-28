use crate::format::*;
use crate::storage::*;
use crate::{Slice, SliceRange, SliceMut};

pub trait Sliceable<T: Element>: Storage<T> {
	#[inline]
	fn slice_axis<A, R>(&self, range: R, _: A)
		-> Slice<T, <A as AxisSelector<R::Size, Self::Rows>>::Result, Self::RowStride, <A as AxisSelector<Self::Cols, R::Size>>::Result, Self::ColStride>
		where A: Axis<Self::Rows, Self::Cols> + AxisSelector<R::Size, Self::Rows> + AxisSelector<Self::Cols, R::Size>,
		      <A as AxisSelector<R::Size, Self::Rows>>::Result: Dim,
		      <A as AxisSelector<Self::Cols, R::Size>>::Result: Dim,
		      R: SliceRange,
	{
		assert!(range.end() <= self.size().get_axis_size::<A>().value(), "Range is out of bounds!");
		unsafe {
			PtrStorage::new(
				match A::axis_type() {
					AxisType::Row => self.as_row_ptr(range.begin()),
					AxisType::Col => self.as_col_ptr(range.begin()),
				},
				Size::new(
					<A as AxisSelector<R::Size, Self::Rows>>::select(range.size(), self.row_dim()),
					<A as AxisSelector<Self::Cols, R::Size>>::select(self.col_dim(), range.size()),
				),
				self.strides()
			).into()
		}
	}

	#[inline]
	fn slice_rows<R: SliceRange>(&self, range: R)
		-> Slice<T, R::Size, Self::RowStride, Self::Cols, Self::ColStride>
	{ self.slice_axis(range, RowAxis) }

	#[inline]
	fn slice_cols<R: SliceRange>(&self, range: R)
		-> Slice<T, Self::Rows, Self::RowStride, R::Size, Self::ColStride>
	{ self.slice_axis(range, ColAxis) }


	#[inline]
	fn slice<RR: SliceRange, CR: SliceRange>(&self, range_rows: RR, range_cols: CR)
		-> Slice<T, RR::Size, Self::RowStride, CR::Size, Self::ColStride>
	{
		assert!(range_cols.end() <= self.cols() && range_rows.end() <= self.rows(), "Range is out of bounds!");
		unsafe {
			PtrStorage::new(
				self.get_ptr(range_rows.begin(), range_cols.begin()),
				Size::new(range_rows.size(), range_cols.size()),
				self.strides()
			).into()
		}
	}

	fn into_slice(&self) -> Slice<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> {
		unsafe {
			PtrStorage::new(
				self.as_ptr(),
				self.size(),
				self.strides()
			).into()
		}
	}
}

pub trait SliceableMut<T: Element>: StorageMut<T> {
	#[inline]
	fn slice_axis_mut<A, R>(&mut self, range: R, _: A)
		-> SliceMut<T, <A as AxisSelector<R::Size, Self::Rows>>::Result, Self::RowStride, <A as AxisSelector<Self::Cols, R::Size>>::Result, Self::ColStride>
		where A: Axis<Self::Rows, Self::Cols> + AxisSelector<R::Size, Self::Rows> + AxisSelector<Self::Cols, R::Size>,
		      <A as AxisSelector<R::Size, Self::Rows>>::Result: Dim,
		      <A as AxisSelector<Self::Cols, R::Size>>::Result: Dim,
		      R: SliceRange,
	{
		assert!(range.end() <= self.size().get_axis_size::<A>().value(), "Range is out of bounds!");
		unsafe {
			PtrStorageMut::new(
				match A::axis_type() {
					AxisType::Row => self.as_row_ptr_mut(range.begin()),
					AxisType::Col => self.as_col_ptr_mut(range.begin()),
				},
				Size::new(
					<A as AxisSelector<R::Size, Self::Rows>>::select(range.size(), self.row_dim()),
					<A as AxisSelector<Self::Cols, R::Size>>::select(self.col_dim(), range.size()),
				),
				self.strides()
			).into()
		}
	}

	#[inline]
	fn slice_rows_mut<R: SliceRange>(&mut self, range: R)
		-> SliceMut<T, R::Size, Self::RowStride, Self::Cols, Self::ColStride>
	{ self.slice_axis_mut(range, RowAxis) }


	#[inline]
	fn slice_cols_mut<R: SliceRange>(&mut self, range: R)
		-> SliceMut<T, Self::Rows, Self::RowStride, R::Size, Self::ColStride>
	{ self.slice_axis_mut(range, ColAxis) }


	#[inline]
	fn slice_mut<RR: SliceRange, CR: SliceRange>(&mut self, range_rows: RR, range_cols: CR)
		-> SliceMut<T, RR::Size, Self::RowStride, CR::Size, Self::ColStride>
	{
		assert!(range_cols.end() <= self.cols() && range_rows.end() <= self.rows(), "Range is out of bounds!");
		unsafe {
			PtrStorageMut::new(
				self.get_ptr_mut(range_rows.begin(), range_cols.begin()),
				Size::new(range_rows.size(), range_cols.size()),
				self.strides()
			).into()
		}
	}

	#[inline]
	fn into_slice_mut(&mut self)
		-> SliceMut<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride>
	{
		unsafe {
			PtrStorageMut::new(
				self.as_ptr_mut(),
				self.size(),
				self.strides()
			).into()
		}
	}
}