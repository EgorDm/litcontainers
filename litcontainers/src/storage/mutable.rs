use crate::format::*;
use crate::storage::*;
use crate::iterator::*;
use std::slice;
use crate::SliceRange;

pub trait StorageMut<T>: Storage<T>
	where T: Scalar
{
	#[inline]
	fn as_ptr_mut(&mut self) -> *mut T;

	#[inline]
	fn as_mut_slice<'b, 'a: 'b>(&'a mut self) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut(self.as_ptr_mut(), self.len()) }
	}

	#[inline]
	fn get_mut(&mut self, r: usize, c: usize) -> &mut T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.get_mut_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_mut_unchecked(&mut self, r: usize, c: usize) -> &mut T {
		&mut *self.as_ptr_mut().offset(self.index(r, c) as isize)
	}

	#[inline]
	fn as_row_ptr_mut(&mut self, p: usize) -> *mut T {
		assert!(p < self.rows(), "Row out of bounds!");
		unsafe { self.as_row_ptr_mut_unchecked(p) }
	}

	#[inline]
	unsafe fn as_row_ptr_mut_unchecked(&mut self, p: usize) -> *mut T { self.as_ptr_mut().offset(self.row_index(p) as isize) }

	#[inline]
	fn as_col_ptr_mut(&mut self, v: usize) -> *mut T {
		assert!(v < self.cols(), "Col out of bounds!");
		unsafe { self.as_col_ptr_mut_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_ptr_mut_unchecked(&mut self, p: usize) -> *mut T { self.as_ptr_mut().offset(self.col_index(p) as isize) }

	// Row Contigious Access Functions
	#[inline]
	fn as_row_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		let size = self.index(v, self.cols() - 1) - self.index(v, 0) + 1;
		unsafe { slice::from_raw_parts_mut(self.as_row_mut_ptr(v), size) }
	}

	#[inline]
	fn as_row_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.rows(), "Row out of bounds!");
		unsafe { self.as_row_mut_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_row_mut_ptr_unchecked(&mut self, v: usize) -> *mut T {
		self.as_ptr_mut().offset(self.row_index(v) as isize)
	}

	// Col Contigious Access Functions
	#[inline]
	fn as_col_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		let size = self.index(self.rows() - 1, v) - self.index(0, v) + 1;
		unsafe { slice::from_raw_parts_mut(self.as_col_mut_ptr(v), size) }
	}

	#[inline]
	fn as_col_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.cols(), "Col out of bounds!");
		unsafe { self.as_col_mut_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_mut_ptr_unchecked(&mut self, v: usize) -> *mut T {
		self.as_ptr_mut().offset(self.col_index(v) as isize)
	}

	// Iterator
	fn as_iter_mut<'a: 'b, 'b>(&'a mut self) -> FullRowIterMut<'b, T, Self> { self.as_row_iter_mut() }

	fn as_row_iter_mut<'a: 'b, 'b>(&'a mut self) -> FullRowIterMut<'b, T, Self> { full_row_iter!(mut self) }

	fn as_row_slice_iter_mut<'a: 'b, 'b>(&'a mut self) -> RowSliceIterMut<'b, T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { RowSliceIterMut::from_storage(self) }

	fn as_row_range_iter_mut<'a: 'b, 'b, RR: SliceRange<Self::Rows>>(&'a mut self, range: RR)
		-> FullIterMut<'a, T, RR::Size, Self::RowStride, Self::ColStride>
	{
		full_row_iter!(mut self, range)
	}

	fn as_col_iter_mut<'a: 'b, 'b>(&'a mut self) -> FullColIterMut<'b, T, Self> { full_col_iter!(mut self) }

	fn as_col_slice_iter_mut<'a: 'b, 'b>(&'a mut self) -> ColSliceIterMut<'b, T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { ColSliceIterMut::from_storage(self) }

	fn as_col_range_iter_mut<'a: 'b, 'b, CR: SliceRange<Self::Cols>>(&'a mut self, range: CR)
		-> FullIterMut<'a, T, CR::Size, Self::ColStride, Self::RowStride>
	{
		full_col_iter!(mut self, range)
	}

	// Special ops
	#[inline]
	fn copy_from<SO: Storage<T>>(&mut self, from: &SO)
	{
		assert!(self.equal_size(from), "Slice is out of bounds!");
		for (t, f) in self.as_iter_mut().zip(from.as_iter()) {
			*t = *f;
		}
	}
}

/*
impl<T, R, C, S> SliceableMut<T, R, C> for S
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C> {}*/
