use crate::format::*;
use crate::storage::*;
#[macro_use]
use crate::iterator::*;
use std::fmt::Debug;
use std::slice;
use crate::slice::{SliceRange};
use crate::{Container, Sliceable};
use std::iter::Cloned;

//type RowItss<S> =

// TODO: implement proper equality?
pub trait Storage<T>: StorageSize + Strided + Debug + Sized + Ownable<T> + Send + Sync
	where T: Element
{
	#[inline]
	fn as_ptr(&self) -> *const T;

	#[inline]
	fn as_slice(&self) -> &[T] {
		unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	#[inline]
	fn get(&self, r: usize, c: usize) -> T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.get_unchecked(r, c) }
	}

	#[inline]
	fn get_ptr(&self, r: usize, c: usize) -> *const T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.as_ptr().offset(self.index(r, c) as isize) }
	}

	#[inline]
	unsafe fn get_unchecked(&self, r: usize, c: usize) -> T {
		*self.as_ptr().offset(self.index(r, c) as isize)
	}

	#[inline]
	fn get_ref(&self, r: usize, c: usize) -> &T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.get_ref_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_ref_unchecked(&self, r: usize, c: usize) -> &T { &*self.as_ptr().offset(self.index(r, c) as isize) }

	#[inline]
	fn as_row_ptr(&self, p: usize) -> *const T {
		assert!(p < self.rows(), "Row out of bounds!");
		unsafe { self.as_row_ptr_unchecked(p) }
	}

	#[inline]
	unsafe fn as_row_ptr_unchecked(&self, p: usize) -> *const T { self.as_ptr().offset(self.row_index(p) as isize) }

	#[inline]
	fn as_col_ptr(&self, v: usize) -> *const T {
		assert!(v < self.cols(), "Col out of bounds!");
		unsafe { self.as_col_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_ptr_unchecked(&self, p: usize) -> *const T { self.as_ptr().offset(self.col_index(p) as isize) }

	// Iterator
	fn iter(&self) -> Cloned<FullRowIter<T, Self>> { self.as_iter().cloned() }

	fn as_iter(&self) -> FullRowIter<T, Self> { self.as_row_iter() }

	fn as_row_iter(&self) -> FullRowIter<T, Self> { full_row_iter!(self) }

	fn as_row_slice_iter(&self) -> RowSliceIter<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { RowSliceIter::from_storage(self) }

	fn as_row_range_iter<RR: SliceRange<Self::Rows>>(&self, range: RR)
		-> FullIter<T, RR::Size, Self::RowStride, Self::ColStride>
	{
		full_row_iter!(self, range)
	}

	fn as_col_iter(&self) -> FullColIter<T, Self> { full_col_iter!(self) }

	fn as_col_slice_iter(&self) -> RowSliceIter<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { RowSliceIter::from_storage(self) }

	fn as_col_range_iter<CR: SliceRange<Self::Cols>>(&self, range: CR)
		-> FullIter<T, CR::Size, Self::ColStride, Self::RowStride>
	{
		full_col_iter!(self, range)
	}
}


impl<T: Scalar, S: Storage<T>> Sliceable<T> for S {}
