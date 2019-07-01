use crate::format::*;
use crate::storage::SizedStorage;
use crate::iterator::*;
use std::fmt::Debug;
use std::slice;


pub trait Storage<T, R, C>: SizedStorage<R, C> + Debug + Sized// + Ownable
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

	// Row Contigious Access Functions
	#[inline]
	fn as_row_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		unsafe { slice::from_raw_parts(self.as_row_ptr(v), self.col_count() * self.col_stride()) }
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
		unsafe { slice::from_raw_parts(self.as_col_ptr(v), self.row_count() * self.row_stride()) }
	}

	#[inline]
	fn as_col_ptr(&self, v: usize) -> *const T {
		assert!(v < self.col_count(), "Col out of bounds!");
		unsafe { self.as_col_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_ptr_unchecked(&self, v: usize) -> *const T { self.get_index_ptr_unchecked(self.col_index(v)) }

	// Iterator
	fn as_row_iter<'a: 'b, 'b>(&'a self) -> RowIterPtr<'b, T, R, C, Self> { RowIterPtr::new(self) }

	fn as_col_iter<'a: 'b, 'b>(&'a self) -> ColIterPtr<'b, T, R, C, Self> { ColIterPtr::new(self) }
}
