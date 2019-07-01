use crate::format::*;
use crate::storage::{Storage, PtrMutStorage};
use crate::iterator::*;
use std::slice;
use crate::slice::{SliceRange, SliceMut};

pub trait StorageMut<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn get_mut(&mut self, r: usize, c: usize) -> &mut T {
		assert!(r < self.row_count(), "Out of range row!");
		assert!(c < self.col_count(), "Out of range col!");
		unsafe { self.get_mut_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_mut_ptr_unchecked(&mut self, r: usize, c: usize) -> *mut T {
		self.get_index_mut_ptr_unchecked(self.index(r, c))
	}

	#[inline]
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T;

	#[inline]
	unsafe fn get_mut_unchecked(&mut self, r: usize, c: usize) -> &mut T {
		self.get_index_mut_ptr_unchecked(self.index(r, c)).as_mut().unwrap()
	}

	// Row Contigious Access Functions
	#[inline]
	fn as_row_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut(self.as_row_mut_ptr(v), self.col_count() * self.col_stride()) }
	}

	#[inline]
	fn as_row_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.row_count(), "Row out of bounds!");
		unsafe { self.as_row_mut_ptr_uch(v) }
	}

	#[inline]
	unsafe fn as_row_mut_ptr_uch(&mut self, v: usize) -> *mut T {
		self.get_index_mut_ptr_unchecked(self.row_index(v))
	}

	// Col Contigious Access Functions
	#[inline]
	fn as_col_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut(self.as_col_mut_ptr(v), self.row_count() * self.row_stride()) }
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
	fn as_row_mut_iter<'a: 'b, 'b>(&'a mut self) -> RowIterMutPtr<'b, T, R, C, Self> { RowIterMutPtr::new(self) }

	fn as_col_mut_iter<'a: 'b, 'b>(&'a mut self) -> ColIterMutPtr<'b, T, R, C, Self> { ColIterMutPtr::new(self) }

	// Slice
	fn slice_rows<'b: 'c, 'c, RR: SliceRange<R>>(&'b mut self, range: RR) -> SliceMut<'c, T, RR::Size, Self::RStride, C, Self::CStride> {
		assert!(range.end() <= self.row_count(), "Slice is out of bounds!");
		//TODO: cound check
		SliceMut::new(unsafe { PtrMutStorage::new(
			self.as_row_mut_ptr(range.begin()),
			range.size(),
			self.col_dim(),
			self.row_stride_dim(),
			self.col_stride_dim(),
		)})
	}

	fn slice_cols<'b: 'c, 'c, CC: SliceRange<C>>(&'b mut self, range: CC) -> SliceMut<'c, T, R, Self::RStride, CC::Size, Self::CStride> {
		assert!(range.end() <= self.col_count(), "Slice is out of bounds!");
		SliceMut::new(unsafe { PtrMutStorage::new(
			self.as_col_mut_ptr(range.begin()),
			self.row_dim(),
			range.size(),
			self.row_stride_dim(),
			self.col_stride_dim(),
		)})
	}
}