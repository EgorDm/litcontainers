use crate::format::*;
use crate::storage::*;
use crate::slice::*;
use std::marker::PhantomData;
use crate::Parallel;
use rayon::iter::{IntoParallelIterator};


pub trait SplittableIterator: Sized + Iterator + ExactSizeIterator + DoubleEndedIterator {
	fn split_at(self, pos: usize) -> (Self, Self);
}

macro_rules! iter_dim_impl {
	(
		struct $Name: ident <$IterDim: ident, $ElementDim: ident> : $StorageType: ty as $StorageTrait: ident as $StorageRef: ty {
			$slice_fn: ident -> $Element: ty,
			$split_fn: ident,
			$count_fn: ident
		}
	) => {
		pub struct $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{
			storage: $StorageType,
			cursor_front: usize,
			cursor_back: usize,
			_phantoms: PhantomData<(&'a (), T, $IterDim)>
		}

		impl<'a, T, $IterDim, RS, CS> $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{
			pub fn from_storage<'b: 'a, S, $ElementDim>(storage: $StorageRef) -> Self
				where S: $StorageTrait<T, R, C, RStride=RS, CStride=CS> + 'b, $ElementDim: Dim
			{
				// TODO: Mayby handy to move decl of this outside macro to reduce parameter count
				Self::new(storage.$slice_fn(0..storage.$count_fn()))
			}

			pub fn new(storage: $StorageType) -> Self {
				let size = storage.$count_fn();
				Self { storage, cursor_front: 0, cursor_back: size - 1, _phantoms: PhantomData }
			}

			pub fn into_par_iter(self) -> Parallel<Self> {
				Parallel::new(self)
			}
		}

		impl<'a, T, $IterDim, RS, CS> SplittableIterator for $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{
			fn split_at(self, pos: usize)
				-> (Self, Self)
			{
				let (l, r) = self.storage.$split_fn(Dynamic::new(pos));
				(Self::new(l), Self::new(r))
			}
		}

		impl<'a, T, $IterDim, RS, CS> ExactSizeIterator for $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{}

		impl<'a, T, $IterDim, RS, CS> Iterator for $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{
			type Item = $Element;

			fn next(&mut self) -> Option<Self::Item> {
				if self.cursor_front <= self.cursor_back {
					let s: *mut _ = &mut self.storage;
					let ret = unsafe { (*s).$slice_fn(self.cursor_front) }; // TODO: missing whole point of split but oke i guess
					self.cursor_front += 1;
					Some(ret)
				} else {
					None
				}
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				let size = if self.cursor_back > self.cursor_front {
					self.cursor_back - self.cursor_front + 1
				} else {
					0
				};
				(size, Some(size))
			}

			fn count(self) -> usize where Self: Sized {
				if self.cursor_back > self.cursor_front {
					self.cursor_back - self.cursor_front + 1
				}  else {
					0
				}
			}
		}

		impl<'a, T, $IterDim, RS, CS> DoubleEndedIterator for $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{
			fn next_back(&mut self) -> Option<Self::Item> {
				if self.cursor_front <= self.cursor_back {
					let s: *mut _ = &mut self.storage;
					let ret = unsafe { (*s).$slice_fn(self.cursor_back) };
					self.cursor_back += 1;
					Some(ret)
				} else {
					None
				}
			}
		}

		impl<'a, T, $IterDim, RS, CS> IntoParallelIterator for $Name<'a, T, $IterDim, RS, CS>
			where T: Scalar + 'a, $IterDim: Dim, RS: Dim, CS: Dim
		{
			type Iter = Parallel<Self>;
			type Item = <Self as Iterator>::Item;

			fn into_par_iter(self) -> Self::Iter { Parallel::new(self) }
		}
	}
}

iter_dim_impl! {
	struct RowSliceIterSplit<C, R> : Slice<'a, T, Dynamic, RS, C, CS> as Storage as &'b S {
		slice_rows -> Slice<'a, T, U1, RS, C, CS>,
		split_at_row,
		row_count
	}
}

iter_dim_impl! {
	struct RowSliceIterSplitMut<C, R> : SliceMut<'a, T, Dynamic, RS, C, CS> as StorageMut as &'b mut S {
		slice_rows_mut -> SliceMut<'a, T, U1, RS, C, CS>,
		split_at_row_mut,
		row_count
	}
}

iter_dim_impl! {
	struct ColSliceIterSplit<R, C> : Slice<'a, T, R, RS, Dynamic, CS> as Storage as &'b S {
		slice_cols -> Slice<'a, T, R, RS, U1, CS>,
		split_at_col,
		col_count
	}
}

iter_dim_impl! {
	struct ColSliceIterSplitMut<R, C> : SliceMut<'a, T, R, RS, Dynamic, CS> as StorageMut as &'b mut S {
		slice_cols_mut -> SliceMut<'a, T, R, RS, U1, CS>,
		split_at_col_mut,
		col_count
	}
}

pub type ParRowSliceIterSplit<'a, T, C, RS, CS> = Parallel<RowSliceIterSplit<'a, T, C, RS, CS>>;
pub type ParRowSliceIterSplitMut<'a, T, C, RS, CS> = Parallel<RowSliceIterSplitMut<'a, T, C, RS, CS>>;
pub type ParColSliceIterSplit<'a, T, C, RS, CS> = Parallel<ColSliceIterSplit<'a, T, C, RS, CS>>;
pub type ParColSliceIterSplitMut<'a, T, C, RS, CS> = Parallel<ColSliceIterSplitMut<'a, T, C, RS, CS>>;

