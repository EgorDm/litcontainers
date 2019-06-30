use crate::format::*;
//use crate::iterator_naive::*;
use std::fmt::Debug;

pub trait SizedStorage<R, C>
	where R: Dim, C: Dim
{
	#[inline]
	fn row_dim(&self) -> R;

	#[inline]
	fn row_count(&self) -> usize { self.row_dim().value() }

	#[inline]
	fn col_dim(&self) -> C;

	#[inline]
	fn col_count(&self) -> usize { self.col_dim().value() }
}

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
	fn get(&self, r: usize, c: usize) -> T {
		assert!(r < self.row_count(), "Out of range row!");
		assert!(c < self.col_count(), "Out of range col!");
		unsafe { self.get_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_unchecked(&self, r: usize, c: usize) -> T {
		*self.get_ref_unchecked(r, c)
	}

	#[inline]
	fn get_ref(&self, r: usize, c: usize) -> &T {
		assert!(r < self.row_count(), "Out of range row!");
		assert!(c < self.col_count(), "Out of range col!");
		unsafe { self.get_ref_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_ref_unchecked(&self, r: usize, c: usize) -> &T;

	#[inline]
	unsafe fn get_ptr_unchecked(&self, r: usize, c: usize) -> *const T {
		&*self.get_ref_unchecked(r, c)
	}

	fn equal_size<TO, RO, RSO, CO, CSO, SO>(&self, other: &SO) -> bool
		where TO: Scalar, RO: Dim, RSO: Dim, CO: Dim, CSO: Dim, SO: Storage<TO, RO, CO, RStride=RSO, CStride=CSO>
	{
		self.row_count() == other.row_count() && self.col_count() == other.col_count()
	}
}

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
	unsafe fn get_mut_unchecked(&mut self, r: usize, c: usize) -> &mut T;

	#[inline]
	unsafe fn get_mut_ptr_unchecked(&mut self, r: usize, c: usize) -> *mut T {
		&mut *self.get_mut_unchecked(r, c)
	}
}

