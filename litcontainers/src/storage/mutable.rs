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
}

impl<T, R, C, S> SliceableMut<T, R, C> for S
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C> {}