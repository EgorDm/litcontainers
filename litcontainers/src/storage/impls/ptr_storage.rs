use crate::format::*;
use std::marker::PhantomData;
use crate::slice::offset::*;
use crate::storage::*;
use std::ops::{Index, IndexMut};

macro_rules! ptr_storage (
	($Name: ident, $Ptr: ty) => {
		#[repr(C)]
		#[derive(Eq, Debug, Clone, PartialEq)]
		pub struct $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			pub(crate) data: $Ptr,
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

		impl<'a, T, R, RS, C, CS> Index<usize> for $Name<'a, T, R, RS, C, CS>
			where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			type Output = T;

			fn index(&self, index: usize) -> &Self::Output {
				assert!(index < self.size());
				unsafe { &*self.get_index_ptr_unchecked(index) }
			}
		}
	}
);

ptr_storage!(PtrStorage, *const T);
ptr_storage!(PtrMutStorage, *mut T);

// TODO: any suggestions?
unsafe impl<'a, T, R, RS, C, CS> Send for PtrStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<'a, T, R, RS, C, CS> Send for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<'a, T, R, RS, C, CS> Sync for PtrStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<'a, T, R, RS, C, CS> Sync for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim {}

impl<'a, T, R, RS, C, CS> StorageMut<T, R, C> for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self.data.offset(i as isize) }

	fn map_inplace<F: FnMut(&mut T)>(&mut self, mut f: F) {
		// TODO: use one with smallest stride in inner loop?
		for row in 0..self.row_count() {
			let mut row_ptr = self.as_row_mut_ptr(row);
			for _ in 0..self.col_count() {
				unsafe {
					f(&mut *row_ptr);
					row_ptr = row_ptr.offset(self.col_stride() as isize);
				}
			}
		}
	}
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

impl<'a, T, R, RS, C, CS> PtrStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row<P: Dim>(self, pos: P)
		-> (PtrStorage<'a, T, P, RS, C, CS>, PtrStorage<'a, T, <R as DimSub<P>>::Output, RS, C, CS>)
		where P: Dim, R: DimSub<P>
	{
		assert!(pos.value() < self.row_count(), "Slice split is out of bounds or contains empty fragment!");
		unsafe {
			(
				PtrStorage::new(
					self.as_row_ptr(0),
					pos.clone(),
					self.col_dim(),
					self.row_stride_dim(),
					self.col_stride_dim(),
				),
				PtrStorage::new(
					self.as_row_ptr(pos.value()),
					R::sub(self.row_dim(), pos),
					self.col_dim(),
					self.row_stride_dim(),
					self.col_stride_dim(),
				)
			)
		}
	}

	pub fn split_at_col<P: Dim>(self, pos: P)
		-> (PtrStorage<'a, T, R, RS, P, CS>, PtrStorage<'a, T, R, RS, <C as DimSub<P>>::Output, CS>)
		where P: Dim, C: DimSub<P>
	{
		assert!(pos.value() < self.col_count(), "Slice split is out of bounds or contains empty fragment!");
		unsafe {
			(
				PtrStorage::new(
					self.as_row_ptr(0),
					self.row_dim(),
					pos.clone(),
					self.row_stride_dim(),
					self.col_stride_dim(),
				),
				PtrStorage::new(
					self.as_row_ptr(pos.value()),
					self.row_dim(),
					C::sub(self.col_dim(), pos),
					self.row_stride_dim(),
					self.col_stride_dim(),
				)
			)
		}
	}
}

impl<'a, T, R, RS, C, CS> PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row_mut<P: Dim>(self, pos: P)
		-> (PtrMutStorage<'a, T, P, RS, C, CS>, PtrMutStorage<'a, T, <R as DimSub<P>>::Output, RS, C, CS>)
		where P: Dim, R: DimSub<P>
	{
		assert!(pos.value() < self.row_count(), "Slice split is out of bounds or contains empty fragment!");
		let mut self_mut = self;
		unsafe {
			(
				PtrMutStorage::new(
					self_mut.as_row_mut_ptr(0),
					pos.clone(),
					self_mut.col_dim(),
					self_mut.row_stride_dim(),
					self_mut.col_stride_dim(),
				),
				PtrMutStorage::new(
					self_mut.as_row_mut_ptr(pos.value()),
					R::sub(self_mut.row_dim(), pos),
					self_mut.col_dim(),
					self_mut.row_stride_dim(),
					self_mut.col_stride_dim(),
				)
			)
		}
	}

	pub fn split_at_col_mut<P: Dim>(self, pos: P)
		-> (PtrMutStorage<'a, T, R, RS, P, CS>, PtrMutStorage<'a, T, R, RS, <C as DimSub<P>>::Output, CS>)
		where P: Dim, C: DimSub<P>
	{
		assert!(pos.value() < self.col_count(), "Slice split is out of bounds or contains empty fragment!");
		let mut self_mut = self;
		unsafe {
			(
				PtrMutStorage::new(
					self_mut.as_row_mut_ptr(0),
					self_mut.row_dim(),
					pos.clone(),
					self_mut.row_stride_dim(),
					self_mut.col_stride_dim(),
				),
				PtrMutStorage::new(
					self_mut.as_row_mut_ptr(pos.value()),
					self_mut.row_dim(),
					C::sub(self_mut.col_dim(), pos),
					self_mut.row_stride_dim(),
					self_mut.col_stride_dim(),
				)
			)
		}
	}
}

impl<'a, T, R, RS, C, CS> IndexMut<usize> for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}
