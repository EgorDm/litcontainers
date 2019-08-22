use crate::format::*;
use std::marker::PhantomData;
use crate::storage::*;
use crate::{Slice, SliceBase, SliceMut};

#[repr(C)]
#[derive(Debug, new)]
pub struct PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	ptr: *mut T,
	size: Size<R, C>,
	stride: Strides<RS, CS>,
}

impl<T, R, RS, C, CS> StorageSize for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type Rows = R;
	type Cols = C;

	fn row_dim(&self) -> Self::Rows { self.size.row_dim() }

	fn col_dim(&self) -> Self::Cols { self.size.col_dim() }
}

impl<T, R, RS, C, CS> Strided for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type RowStride = RS;
	type ColStride = CS;

	#[inline]
	fn row_stride_dim(&self) -> Self::RowStride { self.stride.row_stride_dim() }

	#[inline]
	fn col_stride_dim(&self) -> Self::ColStride { self.stride.col_stride_dim() }
}

impl<T, R, RS, C, CS> Storage<T> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	#[inline]
	fn as_ptr(&self) -> *const T { self.ptr as *const T }
}

impl<T, R, RS, C, CS> StorageMut<T> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn as_ptr_mut(&mut self) -> *mut T { self.ptr }
}

impl<T, R, RS, C, CS> Ownable<T> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type OwnedType = VecStorageRM<T, R, C>;

	#[inline]
	fn owned(self) -> Self::OwnedType { self.clone_owned() }

	#[inline]
	fn clone_owned(&self) -> Self::OwnedType {
		let data = self.as_iter().cloned().collect();
		Self::OwnedType::from_data(self.size(), data)
	}
}

impl<T, RS, C, CS> PtrStorageCore<T, Dynamic, RS, C, CS>
	where T: Element, RS: Dim, C: Dim, CS: Dim
{
	#[inline]
	fn offset_row(&mut self, p: usize) {
		self.ptr = self.as_row_ptr(p) as *mut T;
		self.size.rows = Dynamic::from(self.rows() - p);
	}

	pub fn shift_row_to<S: Storage<T>>(&mut self, s: &S, pos: usize, length: usize)
	{
		assert!(pos + length <= s.rows());
		assert!(self.equal_strides(s));
		assert!(self.cols() == s.cols());
		self.ptr = s.as_row_ptr(pos) as *mut T;
		self.size.rows = Dynamic::new(length);
	}
}

impl<T, R, RS, CS> PtrStorageCore<T, R, RS, Dynamic, CS>
	where T: Element, R: Dim, RS: Dim, CS: Dim
{
	#[inline]
	fn offset_col(&mut self, p: usize) {
		self.ptr = self.as_col_ptr(p) as *mut T;
		self.size.cols = Dynamic::from(self.cols() - p);
	}

	pub fn shift_col_to<S: Storage<T>>(&mut self, s: &S, pos: usize, length: usize)
	{
		assert!(pos + length <= s.cols());
		assert!(self.equal_strides(s));
		assert!(self.rows() == s.rows());
		self.ptr = s.as_col_ptr(pos) as *mut T;
		self.size.cols = Dynamic::new(length);
	}
}

impl<T, R, RS, C, CS> PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row<P: Dim>(mut self, pos: P)
		-> (PtrStorageCore<T, P, RS, C, CS>, PtrStorageCore<T, <R as DimSub<P>>::Output, RS, C, CS>)
		where P: Dim, R: DimSub<P>
	{
		assert!(pos.value() < self.rows(), "Slice split is out of bounds or contains empty fragment!");
		unsafe {
			(
				PtrStorageCore::new(
					self.as_row_ptr_mut(0),
					Size::new(pos.clone(), self.col_dim()),
					self.strides()
				),
				PtrStorageCore::new(
					self.as_row_ptr_mut(pos.value()),
					Size::new(R::sub(self.row_dim(), pos), self.col_dim()),
					self.strides()
				)
			)
		}
	}

	pub fn split_at_col<P: Dim>(mut self, pos: P)
		-> (PtrStorageCore<T, R, RS, P, CS>, PtrStorageCore<T, R, RS, <C as DimSub<P>>::Output, CS>)
		where P: Dim, C: DimSub<P>
	{
		assert!(pos.value() < self.cols(), "Slice split is out of bounds or contains empty fragment!");
		unsafe {
			(
				PtrStorageCore::new(
					self.as_col_ptr_mut(0),
					Size::new(self.row_dim(), pos.clone()),
					self.strides()
				),
				PtrStorageCore::new(
					self.as_col_ptr_mut(pos.value()),
					Size::new(self.row_dim(), C::sub(self.col_dim(), pos)),
					self.strides()
				)
			)
		}
	}
}

unsafe impl<T, R, RS, C, CS> Send for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<T, R, RS, C, CS> Sync for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {}


#[repr(C)]
#[derive(Debug, StorageSize, Strided, Storage, Ownable)]
pub struct PtrStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	storage: PtrStorageCore<T, R, RS, C, CS>,
	_phantoms: PhantomData<&'a ()>,
}

impl<'a, T, R, RS, C, CS> PtrStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub unsafe fn new(ptr: *const T, size: Size<R, C>, strides: Strides<RS, CS>) -> Self {
		Self { storage: PtrStorageCore::new(ptr as *mut T, size, strides), _phantoms: PhantomData }
	}
}

impl<'a, T, R, RS, C, CS> Into<Slice<'a, T, R, RS, C, CS>> for PtrStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {
	fn into(self) -> Slice<'a, T, R, RS, C, CS> { SliceBase::new(self).into() }
}

#[repr(C)]
#[derive(Debug, StorageSize, Strided, Storage, StorageMut, Ownable)]
pub struct PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	storage: PtrStorageCore<T, R, RS, C, CS>,
	_phantoms: PhantomData<&'a ()>,
}

impl<'a, T, R, RS, C, CS> PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub unsafe fn new(ptr: *mut T, size: Size<R, C>, strides: Strides<RS, CS>) -> Self {
		Self { storage: PtrStorageCore::new(ptr, size, strides), _phantoms: PhantomData }
	}
}

impl<'a, T, R, RS, C, CS> Into<SliceMut<'a, T, R, RS, C, CS>> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {
	fn into(self) -> SliceMut<'a, T, R, RS, C, CS> { SliceBase::new(self).into() }
}

impl<'a, T, R, RS, C, CS> InplaceMap<T> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn map_inplace<F: FnMut(&mut T)>(&mut self, mut f: F) {
		if self.rows() / self.row_stride() <= self.cols() / self.col_stride() {
			for row in 0..self.rows() {
				let mut row_ptr = self.as_row_ptr_mut(row);
				for _ in 0..self.cols() {
					unsafe {
						f(&mut *row_ptr);
						row_ptr = row_ptr.offset(self.col_stride() as isize);
					}
				}
			}
		} else {
			for col in 0..self.cols() {
				let mut col_ptr = self.as_col_ptr_mut(col);
				for _ in 0..self.rows() {
					unsafe {
						f(&mut *col_ptr);
						col_ptr = col_ptr.offset(self.row_stride() as isize);
					}
				}
			}
		}
	}
}

impl<'a, T, R, RS, C, CS, U> InplaceZipMap<T, U> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn map_inplace_zip<F: FnMut(&mut T, U), I: Iterator<Item=U>>(&mut self, mut i: I, mut f: F) {
		if self.rows() / self.row_stride() <= self.cols() / self.col_stride() {
			for row in 0..self.rows() {
				let mut row_ptr = self.as_row_ptr_mut(row);
				for _ in 0..self.cols() {
					unsafe {
						f(&mut *row_ptr, i.next().unwrap());
						row_ptr = row_ptr.offset(self.col_stride() as isize);
					}
				}
			}
		} else {
			for col in 0..self.cols() {
				let mut col_ptr = self.as_col_ptr_mut(col);
				for _ in 0..self.rows() {
					unsafe {
						f(&mut *col_ptr, i.next().unwrap());
						col_ptr = col_ptr.offset(self.row_stride() as isize);
					}
				}
			}
		}
	}
}

/*

macro_rules! ptr_storage (
	($Name: ident, $Ptr: ty) => {
		#[repr(C)]
		#[derive(Eq, Debug, Clone, PartialEq)]
		pub struct $Name<'a, T, R, RS, C, CS>
			where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			pub(crate) data: $Ptr,
			row_dim: R,
			col_dim: C,
			row_stride: RS,
			col_stride: CS,
			_phantoms: PhantomData<(&'a ())>
		}

		impl<'a, T, R, RS, C, CS> $Name<'a, T, R, RS, C, CS>
			where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			pub unsafe fn new(data: $Ptr, row_dim: R, col_dim: C, row_stride: RS, col_stride: CS) -> Self {
				Self { data, row_dim, col_dim, row_stride, col_stride, _phantoms: PhantomData }
			}
		}

		impl<'a, T, R, RS, C, CS> SizedStorage<R, C> for $Name<'a, T, R, RS, C, CS>
			where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			fn row_dim(&self) -> R { self.row_dim }

			fn col_dim(&self) -> C { self.col_dim }
		}

		impl<'a, T, R, RS, C, CS> Storage<T, R, C> for $Name<'a, T, R, RS, C, CS>
			where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			type RStride = RS;
			type CStride = CS;

			fn row_stride_dim(&self) -> Self::RStride { self.row_stride }

			fn col_stride_dim(&self) -> Self::CStride { self.col_stride }

			unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.offset(i as isize) }
		}

		impl<'a, T, R, RS, C, CS> Ownable<T, R, C> for $Name<'a, T, R, RS, C, CS>
			where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
		{
			type OwnedType = VecStorageRM<T, R, C>;

			fn owned(self) -> Self::OwnedType { self.clone_owned() }

			fn clone_owned(&self) -> Self::OwnedType {
				let data = self.as_row_iter().cloned().collect();
				Self::OwnedType::from_data(self.row_dim(), self.col_dim(), data)
			}
		}

		impl<'a, T, RS, C, CS> OffsetableRowSlice<T, C> for $Name<'a, T, Dynamic, RS, C, CS>
			where T: Element, RS: Dim, C: Dim, CS: Dim
		{
			#[inline]
			unsafe fn offset_row_unchecked(&mut self, v: usize) {
				self.data = self.data.offset((v * self.row_stride()) as isize);
				self.row_dim = Dynamic::from(self.row_count() - v);
			}
		}

		impl<'a, T, R, RS, CS> OffsetableColSlice<T, R> for $Name<'a, T, R, RS, Dynamic, CS>
			where T: Element, R: Dim, RS: Dim, CS: Dim
		{
			#[inline]
			unsafe fn offset_col_unchecked(&mut self, v: usize) {
				self.data = self.data.offset((v * self.col_stride()) as isize);
				self.col_dim = Dynamic::from(self.col_count() - v);
			}
		}

		impl<'a, T, R, RS, C, CS> Index<usize> for $Name<'a, T, R, RS, C, CS>
			where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
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
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<'a, T, R, RS, C, CS> Send for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<'a, T, R, RS, C, CS> Sync for PtrStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {}

unsafe impl<'a, T, R, RS, C, CS> Sync for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {}

impl<'a, T, R, RS, C, CS> StorageMut<T, R, C> for PtrMutStorage<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self.data.offset(i as isize) }

	fn map_inplace<F: FnMut(&mut T)>(&mut self, mut f: F) {
		// TODO: use one with smallest stride in inner loop?
		if self.row_count() <= self.col_count() {
			for row in 0..self.row_count() {
				let mut row_ptr = self.as_row_mut_ptr(row);
				for _ in 0..self.col_count() {
					unsafe {
						f(&mut *row_ptr);
						row_ptr = row_ptr.offset(self.col_stride() as isize);
					}
				}
			}
		} else {
			for col in 0..self.col_count() {
				let mut col_ptr = self.as_col_mut_ptr(col);
				for _ in 0..self.row_count() {
					unsafe {
						f(&mut *col_ptr);
						col_ptr = col_ptr.offset(self.row_stride() as isize);
					}
				}
			}
		}
	}
}


impl<'a, T, R, RS, CS> PtrStorage<'a, T, R, RS, Dynamic, CS>
	where T: Element, R: Dim, RS: Dim, CS: Dim
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
	where T: Element, R: Dim, RS: Dim, CS: Dim
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
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
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
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
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
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}
*/
