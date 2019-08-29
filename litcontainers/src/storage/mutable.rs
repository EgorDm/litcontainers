use crate::format::*;
use crate::storage::*;
use crate::iterator::*;
use crate::{SliceRange, SliceableMut};
use std::slice;

pub trait StorageMut<T>: Storage<T> + InplaceMap<T> + InplaceMapOrdered<T>
	where T: Element
{
	#[inline]
	fn as_ptr_mut(&mut self) -> *mut T;

	#[inline]
	fn as_slice_mut<'b, 'a: 'b>(&'a mut self) -> &'b mut [T] {
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
		&mut *self.as_ptr_mut().offset(self.get_index(r, c) as isize)
	}

	#[inline]
	fn get_ptr_mut(&mut self, r: usize, c: usize) -> *mut T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.as_ptr_mut().offset(self.get_index(r, c) as isize) }
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
	fn as_col_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.cols(), "Col out of bounds!");
		unsafe { self.as_col_mut_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_mut_ptr_unchecked(&mut self, v: usize) -> *mut T {
		self.as_ptr_mut().offset(self.col_index(v) as isize)
	}

	// Iterator
	fn as_iter_mut(&mut self) -> FullAxisIterMut<T, Self, RowAxis> { self.as_row_iter_mut() }

	fn as_row_iter_mut(&mut self) -> FullAxisIterMut<T, Self, RowAxis> { FullIterMut::from_storage(self, RowAxis) }

	fn as_row_slice_iter_mut(&mut self) -> RowSliceIterMut<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { RowSliceIterMut::from_storage(self) }

	fn as_row_range_iter_mut<RR: SliceRange>(&mut self, range: RR)
		-> FullIterMut<T, RR::Size, Self::RowStride, Self::ColStride>
	{
		FullIterMut::from_storage_range(self, RowAxis, range)
	}

	fn as_col_iter_mut(&mut self) -> FullAxisIterMut<T, Self, ColAxis> { FullIterMut::from_storage(self, ColAxis) }

	fn as_col_slice_iter_mut(&mut self) -> ColSliceIterMut<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { ColSliceIterMut::from_storage(self) }

	fn as_col_range_iter_mut<CR: SliceRange>(&mut self, range: CR)
		-> FullIterMut<T, CR::Size, Self::ColStride, Self::RowStride>
	{
		FullIterMut::from_storage_range(self, ColAxis, range)
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

impl<T: Scalar, S: StorageMut<T>> SliceableMut<T> for S {}