use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use crate::container::Container;
use crate::slice::offset::*;
use std::fmt::{Display, Formatter, Error};
use std::ops::{Index, IndexMut};


/// Slice containing references to scalar values.
pub type Slice<'a, T, R, RS, C, CS> = SliceBase<'a, T, R, C, PtrStorage<'a, T, R, RS, C, CS>>;
/// Slice containing mutable references to scalar values.
pub type SliceMut<'a, T, R, RS, C, CS> = SliceBase<'a, T, R, C, PtrMutStorage<'a, T, R, RS, C, CS>>;

pub type RowSlice<'a, T, R, C> = Slice<'a, T, R, C, C, U1>;
pub type RowSliceMut<'a, T, R, C> = SliceMut<'a, T, R, C, C, U1>;
pub type ColSlice<'a, T, R, C> = Slice<'a, T, R, U1, C, R>;
pub type ColSliceMut<'a, T, R, C> = SliceMut<'a, T, R, U1, C, R>;

/// Container containing references to scalar values.
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(&'a (), T, R, C, S)>
}

impl<'a, T, R, C, S> SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	pub fn new(storage: S) -> Self {
		SliceBase { storage, _phantoms: PhantomData }
	}
}

impl<'a, T, R, C, S> SizedStorage<R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	fn row_dim(&self) -> R { self.storage.row_dim() }

	fn col_dim(&self) -> C { self.storage.col_dim() }
}

impl<'a, T, R, C, S> Storage<T, R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	type RStride = S::RStride;
	type CStride = S::CStride;

	fn row_stride_dim(&self) -> Self::RStride { self.storage.row_stride_dim() }

	fn col_stride_dim(&self) -> Self::CStride { self.storage.col_stride_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.storage.get_index_ptr_unchecked(i) }
}

impl<'a, T, R, C, S> Ownable<T, R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	type OwnedType = Container<T, R, C, S::OwnedType>;

	fn owned(self) -> Self::OwnedType {
		Container {
			storage: self.storage.owned(),
			_phantoms: PhantomData
		}
	}

	fn clone_owned(&self) -> Self::OwnedType {
		Container {
			storage: self.storage.clone_owned(),
			_phantoms: PhantomData
		}
	}
}

impl<'a, T, R, C, S> StorageMut<T, R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self.storage.get_index_mut_ptr_unchecked(i) }

	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace(f) }
}

impl<'a, T, C, S> OffsetableRowSlice<T, C> for SliceBase<'a, T, Dynamic, C, S>
	where T: Scalar, C: Dim, S: StorageMut<T, Dynamic, C> + OffsetableRowSlice<T, C>
{
	unsafe fn offset_row_unchecked(&mut self, v: usize) {
		self.storage.offset_row_unchecked(v)
	}
}

impl<'a, T, R, S> OffsetableColSlice<T, R> for SliceBase<'a, T, R, Dynamic, S>
	where T: Scalar, R: Dim, S: StorageMut<T, R, Dynamic> + OffsetableColSlice<T, R>
{
	unsafe fn offset_col_unchecked(&mut self, v: usize) {
		self.storage.offset_col_unchecked(v)
	}
}

impl<'a, T, R, RS, C, CS> TransposableInplace<'a, T, R, C> for Slice<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn transmute_dims_inplace<RO, CO, RSO, CSO>(self, row_dim: RO, col_dim: CO, row_stride: RSO, col_stride: CSO)
		-> Slice<'a, T, RO, RSO, CO, CSO>
		where RO: Dim, CO: Dim, RSO: Dim, CSO: Dim
	{
		let new_size = (row_dim.value() - 1) * row_stride.value() + (col_dim.value() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		Slice::new(unsafe {
			PtrStorage::new(
				self.storage.data,
				row_dim,
				col_dim,
				row_stride,
				col_stride,
			)
		})
	}

	fn transmute_stride_dims_inplace<RSO, CSO>(self, row_stride: RSO, col_stride: CSO)
		-> Slice<'a, T, R, RSO, C, CSO>
		where RSO: Dim, CSO: Dim
	{
		let new_size = (self.row_count() - 1) * row_stride.value() + (self.col_count() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		Slice::new(unsafe {
			PtrStorage::new(
				self.storage.data,
				self.row_dim(),
				self.col_dim(),
				row_stride,
				col_stride,
			)
		})
	}
}

impl<'a, T, R, RS, C, CS> TransposableInplaceMut<'a, T, R, C> for SliceMut<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn transmute_dims_inplace<RO, CO, RSO, CSO>(self, row_dim: RO, col_dim: CO, row_stride: RSO, col_stride: CSO)
		-> SliceMut<'a, T, RO, RSO, CO, CSO>
		where RO: Dim, CO: Dim, RSO: Dim, CSO: Dim
	{
		let new_size = (row_dim.value() - 1) * row_stride.value() + (col_dim.value() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.storage.data,
				row_dim,
				col_dim,
				row_stride,
				col_stride,
			)
		})
	}

	fn transmute_stride_dims_inplace<RSO, CSO>(self, row_stride: RSO, col_stride: CSO)
		-> SliceMut<'a, T, R, RSO, C, CSO>
		where RSO: Dim, CSO: Dim
	{
		let new_size = (self.row_count() - 1) * row_stride.value() + (self.col_count() - 1) * col_stride.value();
		assert!(new_size < self.row_count() * self.col_count(), "Transmute is out of bounds!");
		SliceMut::new(unsafe {
			PtrMutStorage::new(
				self.storage.data,
				self.row_dim(),
				self.col_dim(),
				row_stride,
				col_stride,
			)
		})
	}
}

impl<'a, T, R, C, S> Display for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}

impl<'a, T, R, C, S> Index<usize> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		assert!(index < self.size());
		unsafe { &*self.get_index_ptr_unchecked(index) }
	}
}

impl<'a, T, R, C, S> IndexMut<usize> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert!(index < self.size());
		unsafe { &mut *self.get_index_mut_ptr_unchecked(index) }
	}
}

impl<'a, T, R, RS, C, CS> Slice<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row<P: Dim>(self, pos: P)
		-> (Slice<'a, T, P, RS, C, CS>, Slice<'a, T, <R as DimSub<P>>::Output, RS, C, CS>)
		where P: Dim, R: DimSub<P>
	{
		let (l, r) = self.storage.split_at_row(pos);
		(Slice::new(l), Slice::new(r))
	}

	pub fn split_at_col<P: Dim>(self, pos: P)
		-> (Slice<'a, T, R, RS, P, CS>, Slice<'a, T, R, RS, <C as DimSub<P>>::Output, CS>)
		where P: Dim, C: DimSub<P>
	{
		let (l, r) = self.storage.split_at_col(pos);
		(Slice::new(l), Slice::new(r))
	}
}

impl<'a, T, R, RS, C, CS> SliceMut<'a, T, R, RS, C, CS>
	where T: Scalar, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row_mut<P: Dim>(self, pos: P)
		-> (SliceMut<'a, T, P, RS, C, CS>, SliceMut<'a, T, <R as DimSub<P>>::Output, RS, C, CS>)
		where P: Dim, R: DimSub<P>
	{
		let (l, r) = self.storage.split_at_row_mut(pos);
		(SliceMut::new(l), SliceMut::new(r))
	}

	pub fn split_at_col_mut<P: Dim>(self, pos: P)
		-> (SliceMut<'a, T, R, RS, P, CS>, SliceMut<'a, T, R, RS, <C as DimSub<P>>::Output, CS>)
		where P: Dim, C: DimSub<P>
	{
		let (l, r) = self.storage.split_at_col_mut(pos);
		(SliceMut::new(l), SliceMut::new(r))
	}
}

