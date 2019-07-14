use crate::format::*;
use crate::storage::*;
use crate::slice::*;
use std::marker::PhantomData;

macro_rules! iter_dim_impl {
	(
		struct $Name: ident : $StorageType: ident as $StorageRef: ty {
			$slice_fn: ident -> $Element: ty, $count_fn: ident
		}
	) => {
		pub struct $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			storage: $StorageRef,
			cursor: usize,
			_phantoms: PhantomData<(&'a (), T, R, C)>
		}

		impl<'a, T, R, C, S> $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			pub fn new(storage: $StorageRef) -> Self {
				Self { storage, cursor: 0, _phantoms: PhantomData }
			}
		}

		impl<'a, T, R, C, S> ExactSizeIterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{}

		impl<'a, T, R, C, S> Iterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			type Item = $Element;

			fn next(&mut self) -> Option<Self::Item> {
				if self.cursor < self.storage.$count_fn() {
					let ret = self.storage.$slice_fn(self.cursor);
					self.cursor += 1;
					Some(ret)
				} else {
					None
				}
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				let size = self.storage.$count_fn() - self.cursor;
				(size, Some(size))
			}

			fn count(self) -> usize where Self: Sized {
				self.size_hint().0
			}
		}
	}
}

iter_dim_impl!{
	struct RowSliceIter: Storage as &'a S {
		slice_rows -> Slice<'a, T, U1, S::RStride, C, S::CStride>, row_count
	}
}

iter_dim_impl!{
	struct ColSliceIter: Storage as &'a S {
		slice_cols -> Slice<'a, T, R, S::RStride, U1, S::CStride>, col_count
	}
}

macro_rules! iter_dim_mut_impl {
	(
		struct $Name: ident : $StorageType: ident as $StorageRef: ty as $StoragePtr: ty {
			$slice_fn: ident -> $Element: ty, $count_fn: ident
		}
	) => {
		pub struct $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C> + 'a
		{
			storage: $StoragePtr,
			cursor: usize,
			_phantoms: PhantomData<(&'a (), T, R, C)>
		}

		impl<'a, T, R, C, S> $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C> + 'a
		{
			pub fn new(storage: $StorageRef) -> Self {
				Self { storage, cursor: 0, _phantoms: PhantomData }
			}
		}

		impl<'a, T, R, C, S> ExactSizeIterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C> + 'a
		{}

		impl<'a, T, R, C, S> Iterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C> + 'a
		{
			type Item = $Element;

			fn next(&mut self) -> Option<Self::Item> {
				if self.cursor < unsafe { (*self.storage).$count_fn() } {
					let ret = unsafe { (*self.storage).$slice_fn(self.cursor) };
					self.cursor += 1;
					Some(ret)
				} else {
					None
				}
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				let size = unsafe { (*self.storage).$count_fn() - self.cursor };
				(size, Some(size))
			}

			fn count(self) -> usize where Self: Sized {
				self.size_hint().0
			}
		}
	}
}


iter_dim_mut_impl!{
	struct RowSliceIterMut: StorageMut as &'a mut S as *mut S {
		slice_rows_mut -> SliceMut<'a, T, U1, S::RStride, C, S::CStride>, row_count
	}
}

iter_dim_mut_impl!{
	struct ColSliceIterMut: StorageMut as &'a mut S as *mut S {
		slice_cols_mut -> SliceMut<'a, T, R, S::RStride, U1, S::CStride>, col_count
	}
}
