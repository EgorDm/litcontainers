use crate::format::*;
use std::marker::PhantomData;
use crate::slice::offset::*;
use crate::storage::*;

macro_rules! ptr_storage (
	($Name: ident, $Ptr: ty) => {
		#[repr(C)]
		#[derive(Eq, Debug, Clone, PartialEq)]
		pub struct $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			data: $Ptr,
			row_dim: R,
			col_dim: C,
			row_stride: RS,
			col_stride: CS,
			_phantoms: PhantomData<(&'a ())>
		}

		impl<'a, T, R, RS, C, CS> $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			pub unsafe fn new(data: $Ptr, row_dim: R, col_dim: C, row_stride: RS, col_stride: CS) -> Self {
				Self { data, row_dim, col_dim, row_stride, col_stride, _phantoms: PhantomData }
			}
		}

		impl<'a, T, R, RS, C, CS> SizedStorage<R, C> for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			fn row_dim(&self) -> R { self.row_dim }

			fn col_dim(&self) -> C { self.col_dim }
		}

		impl<'a, T, R, RS, C, CS> Storage<T, R, C> for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			type RStride = RS;
			type CStride = CS;

			fn row_stride_dim(&self) -> Self::RStride { self.row_stride }

			fn col_stride_dim(&self) -> Self::CStride { self.col_stride }

			unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.offset(i as isize) }
		}

		impl<'a, T, R, RS, C, CS> Ownable<T, R, C> for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			type OwnedType = VecStorageRM<T, R, C>;

			fn owned(self) -> Self::OwnedType { self.clone_owned() }

			fn clone_owned(&self) -> Self::OwnedType {
				let data = self.as_row_iter().cloned().collect();
				Self::OwnedType::from_data(self.row_dim(), self.col_dim(), data)
			}
		}

		impl<'a, T, RS, C, CS> OffsetableRowSlice<T, C> for $Name<'a, T, Dynamic, RS, C, CS>
			where T: Scalar, RS: Dim, C: Dim, CS: Dim
		{
			#[inline]
			unsafe fn offset_row_unchecked(&mut self, v: usize) {
				self.data = self.data.offset((v * self.row_stride()) as isize);
				self.row_dim = Dynamic::from(self.row_count() - v);
			}
		}

		impl<'a, T, R, RS, CS> OffsetableColSlice<T, R> for $Name<'a, T, R, RS, Dynamic, CS>
			where T: Scalar, R: Dim, RS: Dim, CS: Dim
		{
			#[inline]
			unsafe fn offset_col_unchecked(&mut self, v: usize) {
				self.data = self.data.offset((v * self.col_stride()) as isize);
				self.col_dim = Dynamic::from(self.col_count() - v);
			}
		}
	}
);

ptr_storage!(PtrStorage, *const T);
ptr_storage!(PtrMutStorage, *mut T);

impl<'a, T, R, RS, C, CS> StorageMut<T, R, C> for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self.data.offset(i as isize) }
}


impl<'a, T, R, RS, CS> PtrStorage<'a, T, R, RS, Dynamic, CS>
	where T: Scalar, R: Dim, RS: Dim, CS: Dim
{
	pub fn shift_col_to<S, RO, CO>(&mut self, storage: &S, col_offset: usize, col_count: usize)
		where RO: Dim, CO: Dim,
			S: Storage<T, RO, CO, RStride=<Self as Storage<T, R, Dynamic>>::RStride, CStride=<Self as Storage<T, R, Dynamic>>::CStride>
	{
		assert!(col_offset + col_count <= storage.col_count());
		assert!(self.row_count() == storage.row_count());
		self.col_dim = Dynamic::new(col_count);
		self.data = storage.as_col_ptr(col_offset);
	}
}

impl<'a, T, R, RS, CS> PtrMutStorage<'a, T, R, RS, Dynamic, CS>
	where T: Scalar, R: Dim, RS: Dim, CS: Dim
{
	pub fn shift_col_to<S, RO, CO>(&mut self, storage: &mut S, col_offset: usize, col_count: usize)
		where RO: Dim, CO: Dim, S: StorageMut<T, RO, CO>
		+ Storage<T, RO, CO, RStride=<Self as Storage<T, R, Dynamic>>::RStride, CStride=<Self as Storage<T, R, Dynamic>>::CStride>
	{
		assert!(col_offset + col_count <= storage.col_count());
		assert!(self.row_count() == storage.row_count());
		self.col_dim = Dynamic::new(col_count);
		self.data = storage.as_col_mut_ptr(col_offset);
	}
}