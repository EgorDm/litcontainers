use crate::format::*;
use crate::storage::{SizedStorage, Ownable, PtrStorage};
use crate::iterator::*;
use std::fmt::Debug;
use std::slice;
use crate::slice::{SliceRange, Slice};


pub trait Storage<T, R, C>: SizedStorage<R, C> + Debug + Sized + Ownable<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type RStride: Dim;
	type CStride: Dim;

	#[inline]
	fn scalar_type(&self) -> ScalarType { T::get_scalar_type() }

	#[inline]
	fn row_stride_dim(&self) -> Self::RStride;

	#[inline]
	fn row_stride(&self) -> usize { self.row_stride_dim().value() }

	#[inline]
	fn row_index(&self, p: usize) -> usize { p * self.row_stride() }

	#[inline]
	fn col_stride_dim(&self) -> Self::CStride;

	#[inline]
	fn col_stride(&self) -> usize { self.col_stride_dim().value() }

	#[inline]
	fn col_index(&self, p: usize) -> usize { p * self.col_stride() }

	#[inline]
	fn index(&self, r: usize, c: usize) -> usize { r * self.row_stride() + c * self.col_stride() }

	#[inline]
	unsafe fn get_ptr_unchecked(&self, r: usize, c: usize) -> *const T {
		self.get_index_ptr_unchecked(self.index(r, c))
	}

	#[inline]
	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T;

	#[inline]
	fn get(&self, r: usize, c: usize) -> T {
		assert!(r < self.row_count(), "Out of range row!");
		assert!(c < self.col_count(), "Out of range col!");
		unsafe { self.get_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_unchecked(&self, r: usize, c: usize) -> T {
		*self.get_ptr_unchecked(r, c)
	}

	#[inline]
	fn get_ref(&self, r: usize, c: usize) -> &T {
		assert!(r < self.row_count(), "Out of range row!");
		assert!(c < self.col_count(), "Out of range col!");
		unsafe { self.get_ref_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_ref_unchecked(&self, r: usize, c: usize) -> &T { self.get_ptr_unchecked(r, c).as_ref().unwrap() }

	#[inline]
	fn as_slice<'b, 'a: 'b>(&'a self) -> &'b [T] {
		unsafe { slice::from_raw_parts(self.get_index_ptr_unchecked(0), self.row_count() * self.col_count()) }
	}

	// Row Contigious Access Functions
	#[inline]
	fn as_row_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		let size = self.index(v, self.col_count() - 1) - self.index(v, 0) + 1;
		unsafe { slice::from_raw_parts(self.as_row_ptr(v), size) }
	}

	#[inline]
	fn as_row_ptr(&self, v: usize) -> *const T {
		assert!(v < self.row_count(), "Row out of bounds!");
		unsafe { self.as_row_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_row_ptr_unchecked(&self, v: usize) -> *const T { self.get_index_ptr_unchecked(self.row_index(v)) }

	// Col Contigious Access Functions
	#[inline]
	fn as_col_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		let size = self.index(self.row_count() - 1, v) - self.index(0, v) + 1;
		unsafe { slice::from_raw_parts(self.as_col_ptr(v), size) }
	}

	#[inline]
	fn as_col_ptr(&self, v: usize) -> *const T {
		assert!(v < self.col_count(), "Col out of bounds!");
		unsafe { self.as_col_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_ptr_unchecked(&self, v: usize) -> *const T { self.get_index_ptr_unchecked(self.col_index(v)) }

	// Iterator
	fn as_iter<'a: 'b, 'b>(&'a self) -> RowIterPtr<'b, T, R, C, Self> { self.as_row_iter() }

	fn as_row_iter<'a: 'b, 'b>(&'a self) -> RowIterPtr<'b, T, R, C, Self> { RowIterPtr::new(self) }

	fn as_row_slice_iter<'a: 'b, 'b, RR: SliceRange<R>>(&'a self, range: RR) -> RowIterPtr<'b, T, R, C, Self> {
		RowIterPtr::from_range(self, range.begin(), range.end())
	}

	fn as_col_iter<'a: 'b, 'b>(&'a self) -> ColIterPtr<'b, T, R, C, Self> { ColIterPtr::new(self) }

	fn as_col_slice_iter<'a: 'b, 'b, CC: SliceRange<C>>(&'a self, range: CC) -> ColIterPtr<'b, T, R, C, Self> {
		ColIterPtr::from_range(self, range.begin(), range.end())
	}

	// Slice
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
	fn slice_cols<'b: 'c, 'c, CC: SliceRange<C>>(&'b self, range: CC) -> Slice<'c, T, R, Self::RStride, CC::Size, Self::CStride> {
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
}
