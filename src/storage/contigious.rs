use crate::format::*;
use super::storage::*;
use std::slice;

pub trait RowContigious<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn as_row_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		unsafe { slice::from_raw_parts(self.as_row_ptr(v), self.col_count() * self.col_stride()) }
	}

	#[inline]
	fn as_row_ptr(&self, v: usize) -> *const T {
		assert!(v < self.row_count(), "Row out of bounds!");
		unsafe { self.as_row_ptr_uch(v) }
	}

	#[inline]
	unsafe fn as_row_ptr_uch(&self, v: usize) -> *const T;
}

pub trait RowContigiousMut<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
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
	unsafe fn as_row_mut_ptr_uch(&mut self, v: usize) -> *mut T;
}

pub trait ColContigious<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn as_col_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		unsafe { slice::from_raw_parts(self.as_col_ptr(v), self.row_count() * self.row_stride()) }
	}

	#[inline]
	fn as_col_ptr(&self, v: usize) -> *const T {
		assert!(v < self.col_count(), "Col out of bounds!");
		unsafe { self.as_col_ptr_uch(v) }
	}

	#[inline]
	unsafe fn as_col_ptr_uch(&self, v: usize) -> *const T;
}

pub trait ColContigiousMut<T, R, C>: Storage<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn as_col_mut_slice<'b, 'a: 'b>(&'a mut self, v: usize) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut(self.as_col_mut_ptr(v), self.row_count() * self.row_stride()) }
	}

	#[inline]
	fn as_col_mut_ptr(&mut self, v: usize) -> *mut T {
		assert!(v < self.col_count(), "Col out of bounds!");
		unsafe { self.as_col_mut_ptr_uch(v) }
	}

	#[inline]
	unsafe fn as_col_mut_ptr_uch(&mut self, v: usize) -> *mut T;
}