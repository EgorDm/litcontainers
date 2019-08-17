use crate::format::*;
use crate::storage::{SizedStorage, Ownable, StorageMut};
use crate::iterator::*;
use std::fmt::Debug;
use std::slice;
use crate::slice::{SliceRange, Sliceable};
use std::ops::Index;

// TODO: implement proper equality?
pub trait Storage<T, R, C>: SizedStorage<R, C> + Debug + Sized + Ownable<T, R, C> + Send + Sync + Index<usize, Output=T>
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
	fn calc_index(&self, r: usize, c: usize) -> usize { r * self.row_stride() + c * self.col_stride() }

	#[inline]
	fn size(&self) -> usize { self.row_count() * self.col_count() }

	#[inline]
	fn as_ptr(&self) -> *const T { unsafe { self.get_index_ptr_unchecked(0) } }

	#[inline]
	unsafe fn get_ptr_unchecked(&self, r: usize, c: usize) -> *const T {
		self.get_index_ptr_unchecked(self.calc_index(r, c))
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
	fn row_index_span(&self, row: usize) -> usize {
		self.calc_index(row, self.col_count() - 1) - self.calc_index(row, 0) + 1
	}

	#[inline]
	fn as_row_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		let size = self.row_index_span(v);
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
	fn col_index_span(&self, col: usize) -> usize {
		self.calc_index(self.row_count() - 1, col) - self.calc_index(0, col) + 1
	}

	#[inline]
	fn as_col_slice<'b, 'a: 'b>(&'a self, v: usize) -> &'b [T] {
		let size = self.col_index_span(v);
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
	fn iter(self) -> RowIterPtrOwned<T, R, C, Self> { self.row_iter() }

	fn as_iter<'a: 'b, 'b>(&'a self) -> RowIterPtr<'b, T, R, C, Self> { self.as_row_iter() }

	fn row_iter(self) -> RowIterPtrOwned<T, R, C, Self> { RowIterPtrOwned::new(self) }

	fn as_row_iter<'a: 'b, 'b>(&'a self) -> RowIterPtr<'b, T, R, C, Self> { RowIterPtr::new(self) }

	fn slice_rows_as_iter<'a: 'b, 'b, RR: SliceRange<R>>(&'a self, range: RR) -> RowIterPtr<'b, T, R, C, Self> {
		RowIterPtr::from_range(self, range.begin(), range.end())
	}

	fn as_row_slice_iter<'a: 'b, 'b>(&'a self) -> RowSliceIter<'b, T, R, C, Self> { RowSliceIter::new(self) }

	fn as_row_slice_par_iter<'a: 'b, 'b>(&'a self) -> ParRowSliceIterSplit<'b, T, C, Self::RStride, Self::CStride> {
		Parallel::new(RowSliceIterSplit::from_storage(self))
	}

	fn col_iter(self) -> ColIterPtrOwned<T, R, C, Self> { ColIterPtrOwned::new(self) }

	fn as_col_iter<'a: 'b, 'b>(&'a self) -> ColIterPtr<'b, T, R, C, Self> { ColIterPtr::new(self) }

	fn slice_cols_as_iter<'a: 'b, 'b, CC: SliceRange<C>>(&'a self, range: CC) -> ColIterPtr<'b, T, R, C, Self> {
		ColIterPtr::from_range(self, range.begin(), range.end())
	}

	fn as_col_slice_iter<'a: 'b, 'b>(&'a self) -> ColSliceIter<'b, T, R, C, Self> { ColSliceIter::new(self) }

	fn as_col_slice_par_iter<'a: 'b, 'b>(&'a self) -> ParColSliceIterSplit<'b, T, R, Self::RStride, Self::CStride> {
		Parallel::new(ColSliceIterSplit::from_storage(self))
	}

	fn flip(&self) -> Self::OwnedType {
		let mut ret = self.clone_owned();
		for (i, out_elem) in ret.as_iter_mut().enumerate() {
			*out_elem = self[self.size() - 1 - i]
		}
		ret
	}

	fn flip_rows(&self) -> Self::OwnedType {
		let mut ret = self.clone_owned();
		for (mut out_row, row) in ret.as_row_slice_mut_iter().zip(self.as_row_slice_iter()) {
			for (i, out_col) in out_row.as_iter_mut().enumerate() {
				*out_col = row.get(0, self.col_count() - 1 - i);
			}
		}
		ret
	}

	fn flip_cols(&self) -> Self::OwnedType {
		let mut ret = self.clone_owned();
		for (mut out_col, col) in ret.as_col_slice_mut_iter().zip(self.as_col_slice_iter()) {
			for (i, out_row) in out_col.as_iter_mut().enumerate() {
				*out_row = col.get(self.row_count() - 1 - i, 0);
			}
		}
		ret
	}
}

impl<T, R, C, S> Sliceable<T, R, C> for S
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C> {}