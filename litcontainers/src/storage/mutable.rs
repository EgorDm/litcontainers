use crate::format::*;
use crate::storage::{Storage};
use crate::iterator::*;
use std::slice;
use crate::slice::{SliceRange, SliceableMut};
use std::ops::IndexMut;

pub trait StorageMut<T, R, C>: Storage<T, R, C> + IndexMut<usize>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn as_mut_slice<'b, 'a: 'b>(&'a mut self) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut(self.get_index_mut_ptr_unchecked(0), self.row_count() * self.col_count()) }
	}

	#[inline]
	fn get_mut(&mut self, r: usize, c: usize) -> &mut T {
		assert!(r < self.row_count(), "Out of range row!");
		assert!(c < self.col_count(), "Out of range col!");
		unsafe { self.get_mut_unchecked(r, c) }
	}

	#[inline]
	fn get_mut_at(&mut self, i: usize) -> &mut T {
		assert!(i < self.size(), "Index out of bounds!");
		unsafe { self.get_index_mut_ptr_unchecked(i).as_mut().unwrap() }
	}

	#[inline]
	unsafe fn get_mut_ptr_unchecked(&mut self, r: usize, c: usize) -> *mut T {
		self.get_index_mut_ptr_unchecked(self.calc_index(r, c))
	}

	#[inline]
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T;

	#[inline]
	unsafe fn get_mut_unchecked(&mut self, r: usize, c: usize) -> &mut T {
		self.get_index_mut_ptr_unchecked(self.calc_index(r, c)).as_mut().unwrap()
	}

	// Row Contigious Access Functions
	#[inline]
	fn as_row_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		let size = self.calc_index(v, self.col_count() - 1) - self.calc_index(v, 0) + 1;
		unsafe { slice::from_raw_parts_mut(self.as_row_mut_ptr(v), size) }
	}

	#[inline]
	fn as_row_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.row_count(), "Row out of bounds!");
		unsafe { self.as_row_mut_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_row_mut_ptr_unchecked(&mut self, v: usize) -> *mut T {
		self.get_index_mut_ptr_unchecked(self.row_index(v))
	}

	// Col Contigious Access Functions
	#[inline]
	fn as_col_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		let size = self.calc_index(self.row_count() - 1, v) - self.calc_index(0, v) + 1;
		unsafe { slice::from_raw_parts_mut(self.as_col_mut_ptr(v), size) }
	}

	#[inline]
	fn as_col_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.col_count(), "Col out of bounds!");
		unsafe { self.as_col_mut_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_mut_ptr_unchecked(&mut self, v: usize) -> *mut T {
		self.get_index_mut_ptr_unchecked(self.col_index(v))
	}

	// Iterator
	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F);

	fn mapv_inplace<F: FnMut(T) -> T>(&mut self, mut f: F) {
		self.map_inplace(|v| *v = f(*v))
	}

	fn as_iter_mut<'a: 'b, 'b>(&'a mut self) -> RowIterMutPtr<'b, T, R, C, Self> { self.as_row_mut_iter() }

	fn as_row_mut_iter<'a: 'b, 'b>(&'a mut self) -> RowIterMutPtr<'b, T, R, C, Self> { RowIterMutPtr::new(self) }

	fn slice_rows_as_mut_iter<'a: 'b, 'b, RR: SliceRange<R>>(&'a mut self, range: RR) -> RowIterMutPtr<'b, T, R, C, Self> {
		RowIterMutPtr::from_range(self, range.begin(), range.end())
	}

	fn as_row_slice_mut_iter<'a: 'b, 'b>(&'a mut self) -> RowSliceIterMut<'b, T, R, C, Self> { RowSliceIterMut::new(self) }

	fn as_row_slice_par_mut_iter<'a: 'b, 'b>(&'a mut self) -> ParRowSliceIterSplitMut<'b, T, C, Self::RStride, Self::CStride> {
		ParRowSliceIterSplitMut::from_storage(self)
	}

	fn as_col_mut_iter<'a: 'b, 'b>(&'a mut self) -> ColIterMutPtr<'b, T, R, C, Self> { ColIterMutPtr::new(self) }

	fn slice_cols_as_mut_iter<'a: 'b, 'b, CC: SliceRange<C>>(&'a mut self, range: CC) -> ColIterMutPtr<'b, T, R, C, Self> {
		ColIterMutPtr::from_range(self, range.begin(), range.end())
	}

	fn as_col_slice_mut_iter<'a: 'b, 'b>(&'a mut self) -> ColSliceIterMut<'b, T, R, C, Self> { ColSliceIterMut::new(self) }

	fn as_col_slice_par_mut_iter<'a: 'b, 'b>(&'a mut self) -> ParColSliceIterSplitMut<'b, T, R, Self::RStride, Self::CStride> {
		ParColSliceIterSplitMut::from_storage(self)
	}

	// Special ops
	#[inline]
	fn copy_from<RO, CO, SO>(&mut self, from: &SO)
		where RO: Dim, CO: Dim, SO: Storage<T, RO, CO>
	{
		assert!(self.equal_size(from), "Slice is out of bounds!");
		for (t, f) in self.as_row_mut_iter().zip(from.as_row_iter()) {
			*t = *f;
		}
	}

	#[inline]
	fn fill(&mut self, value: T)
	{
		for o in self.as_iter_mut() { *o = value; }
	}

	// TODO: remove these joins. Use macro instead
	fn join_cols<SL, CL, SR, CR>(self, l: &SL, r: &SR) -> Self
		where SL: Storage<T, R, CL>, SR: Storage<T, R, CR>, CL: Dim + DimAdd<CR, Output=C>, CR: Dim
	{
		unsafe { self.join_cols_unchecked(l, r) }
	}

	unsafe fn join_cols_unchecked<SL, CL, SR, CR>(self, l: &SL, r: &SR) -> Self
		where SL: Storage<T, R, CL>, SR: Storage<T, R, CR>, CL: Dim, CR: Dim
	{
		assert_eq!(l.col_count() + r.col_count(), self.col_count(), "Columns must add up");
		let mut self_mut = self;
		for (mut out_row, (l_row, r_row)) in self_mut.as_row_slice_mut_iter()
			.zip(l.as_row_slice_iter().zip(r.as_row_slice_iter())) {

			for (out_col, in_col) in out_row.as_iter_mut().zip(l_row.iter().chain(r_row.iter())) {
				*out_col = in_col;
			}
		}
		self_mut
	}

	fn join_rows<SL, RL, SR, RR>(self, l: &SL, r: &SR) -> Self
		where SL: Storage<T, RL, C>, SR: Storage<T, RR, C>, RL: Dim + DimAdd<RR, Output=R>, RR: Dim
	{
		unsafe { self.join_rows_unchecked(l, r) }
	}

	unsafe fn join_rows_unchecked<SL, RL, SR, RR>(self, l: &SL, r: &SR) -> Self
		where SL: Storage<T, RL, C>, SR: Storage<T, RR, C>, RL: Dim + DimAdd<RR, Output=R>, RR: Dim
	{
		assert_eq!(l.row_count() + r.row_count(), self.row_count(), "Columns must add up");
		let mut self_mut = self;
		for (mut out_col, (l_col, r_col)) in self_mut.as_col_slice_mut_iter()
			.zip(l.as_col_slice_iter().zip(r.as_col_slice_iter())) {

			for (out_row, in_row) in out_col.as_iter_mut().zip(l_col.iter().chain(r_col.iter())) {
				*out_row = in_row;
			}
		}
		self_mut
	}
}

impl<T, R, C, S> SliceableMut<T, R, C> for S
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C> {}