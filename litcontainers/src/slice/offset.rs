use crate::{Storage, Dynamic, Scalar, Dim};

pub trait OffsetableRowSlice<T, C>: Storage<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	#[inline]
	fn offset_row(&mut self, v: usize) {
		assert!(v < self.row_count(), "Offset is out of bounds");
		unsafe { self.offset_row_unchecked(v) };
	}

	#[inline]
	unsafe fn offset_row_unchecked(&mut self, v: usize);
}

pub trait OffsetableColSlice<T, R>: Storage<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	#[inline]
	fn offset_col(&mut self, v: usize) {
		assert!(v < self.col_count(), "Offset is out of bounds");
		unsafe { self.offset_col_unchecked(v) };
	}

	#[inline]
	unsafe fn offset_col_unchecked(&mut self, v: usize);
}
