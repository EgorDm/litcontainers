use crate::format::*;
use crate::slice::{Slice, SliceMut, SliceBase};
use crate::storage::*;
use super::axis::*;
use std::marker::PhantomData;

macro_rules! slice_iter (
	($NameCore: ident, $Name: ident, $Stride: ident, $iter_fn: expr => $StorageRef: ty as $StorageType: ident, $Slice: ty) => {
		#[derive(Debug)]
		pub struct $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			iter: AxisIterRaw<T, $Stride>,
			size: Size<R, C>,
			stride: Strides<RS, CS>,
			_phantoms: PhantomData<&'a ()>,
		}

		impl<'a, T, R, RS, C, CS> $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			fn new(iter: AxisIterRaw<T, $Stride>, size: Size<R, C>, stride: Strides<RS, CS>) -> Self {
				Self { iter, size, stride, _phantoms: PhantomData }
			}

			pub fn from_storage<S>(s: $StorageRef) -> Self
				where S: $StorageType<T> + StorageSize<Rows=R, Cols=C> + Strided<RowStride=RS, ColStride=CS>
			{
				Self::new($iter_fn(s, 0).iter, s.size(), s.strides())
			}

			fn split_at(self, pos: usize) -> (Self, Self) {
				let (left, right) = self.iter.split_at(pos);
				(
					Self::new(left, self.size.size(), self.stride.clone()),
					Self::new(right, self.size, self.stride),
				)
			}
		}

		impl<'a, T, R, RS, C, CS> ExactSizeIterator for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim {}

		impl<'a, T, R, RS, C, CS> Iterator for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			type Item = $Slice;

			fn next(&mut self) -> Option<Self::Item> {
				self.iter.next().map(|v| self.make_slice(v))
			}
		}

		impl<'a, T, R, RS, C, CS> DoubleEndedIterator for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			fn next_back(&mut self) -> Option<Self::Item> {
				self.iter.next_back().map(|v| self.make_slice(v))
			}
		}
	}
);

slice_iter!(RowSliceIterCore, RowSliceIter, RS, col_iter => &'a S as Storage, Slice<'a, T, U1, C, C, CS>);
impl<'a, T, R, RS, C, CS> RowSliceIter<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn make_slice(&self, ptr: *mut T) -> <Self as Iterator>::Item {
		SliceBase::new(
			unsafe {
				PtrStorage::new(
					ptr as *const T,
					Size::new(U1, self.size.col_dim()),
					Strides::new(self.size.col_dim(), self.stride.col_stride_dim())
				)
			}
		).into()
	}
}

slice_iter!(RowSliceIterMutCore, RowSliceIterMut, RS, col_iter => &'a mut S as StorageMut, SliceMut<'a, T, U1, C, C, CS>);
impl<'a, T, R, RS, C, CS> RowSliceIterMut<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn make_slice(&self, ptr: *mut T) -> <Self as Iterator>::Item {
		unsafe {
			PtrStorageMut::new(
				ptr,
				Size::new(U1, self.size.col_dim()),
				Strides::new(self.size.col_dim(), self.stride.col_stride_dim())
			).into()
		}
	}
}

slice_iter!(ColSliceIterCore, ColSliceIter, CS, row_iter => &'a S as Storage, Slice<'a, T, R, RS, U1, R>);
impl<'a, T, R, RS, C, CS> ColSliceIter<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn make_slice(&self, ptr: *mut T) -> <Self as Iterator>::Item {
		unsafe {
			PtrStorage::new(
				ptr as *const T,
				Size::new(self.size.row_dim(), U1),
				Strides::new(self.stride.row_stride_dim(), self.size.row_dim())
			).into()
		}
	}
}

slice_iter!(ColSliceIterMutCore, ColSliceIterMut, CS, row_iter_mut => &'a mut S as StorageMut, SliceMut<'a, T, R, RS, U1, R>);
impl<'a, T, R, RS, C, CS> ColSliceIterMut<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn make_slice(&self, ptr: *mut T) -> <Self as Iterator>::Item {
		unsafe {
			PtrStorageMut::new(
				ptr,
				Size::new(self.size.row_dim(), U1),
				Strides::new(self.stride.row_stride_dim(), self.size.row_dim())
			).into()
		}
	}
}