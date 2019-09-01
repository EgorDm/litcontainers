use crate::format::*;
use std::marker::PhantomData;
use crate::storage::*;
use crate::{Slice, SliceBase, SliceMut, Container};
use std::ops::{Index, IndexMut};

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

impl<T, R, RS, C, CS> InplaceMap<T> for PtrStorageCore<T, R, RS, C, CS>
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

impl<T, R, RS, C, CS> InplaceForeach<T> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn foreach<F: FnMut(&T)>(&self, mut f: F) {
		if self.rows() / self.row_stride() <= self.cols() / self.col_stride() {
			for row in 0..self.rows() {
				let mut row_ptr = self.as_row_ptr(row);
				for _ in 0..self.cols() {
					unsafe {
						f(&*row_ptr);
						row_ptr = row_ptr.offset(self.col_stride() as isize);
					}
				}
			}
		} else {
			for col in 0..self.cols() {
				let mut col_ptr = self.as_col_ptr(col);
				for _ in 0..self.rows() {
					unsafe {
						f(&*col_ptr);
						col_ptr = col_ptr.offset(self.row_stride() as isize);
					}
				}
			}
		}
	}
}

impl<T, R, RS, C, CS> InplaceMapOrdered<T> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn map_inplace_ordered<F: FnMut(&mut T)>(&mut self, mut f: F) {
		for v in self.as_iter_mut() { f(v) }
	}
}

impl<T, R, RS, C, CS> Ownable<T> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	type OwnedType = VecStorageRM<T, R, C>;

	#[inline]
	fn owned(self) -> Container<T, Self::OwnedType> { self.clone_owned().into() }

	#[inline]
	fn clone_owned(&self) -> Container<T, Self::OwnedType> {
		let data = self.as_iter().cloned().collect();
		Self::OwnedType::from_data(self.size(), data).into()
	}
}

impl<T, R, RS, C, CS> Index<usize> for PtrStorageCore<T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		let r = index / self.cols();
		let c = index % self.cols();
		self.get_ref(r, c)
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
		assert_eq!(self.col_stride(), s.col_stride());
		assert_eq!(self.rows(), s.rows());
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

	pub fn split_at_col<P: Dim>(mut self, pos: P)
		-> (PtrStorageCore<T, R, RS, P, CS>, PtrStorageCore<T, R, RS, <C as DimSub<P>>::Output, CS>)
		where P: Dim, C: DimSub<P>
	{
		assert!(pos.value() < self.cols(), "Slice split is out of bounds or contains empty fragment!");
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

impl<'a, T, RS, C, CS> PtrStorage<'a, T, Dynamic, RS, C, CS>
	where T: Element, RS: Dim, C: Dim, CS: Dim
{
	#[inline]
	pub fn offset_row(&mut self, p: usize) { self.storage.offset_row(p) }

	pub fn shift_row_to<S: Storage<T>>(&mut self, s: &S, pos: usize, length: usize) { self.storage.shift_row_to(s, pos, length)}
}

impl<'a, T, R, RS, CS> PtrStorage<'a, T, R, RS, Dynamic, CS>
	where T: Element, RS: Dim, R: Dim, CS: Dim
{
	#[inline]
	pub fn offset_col(&mut self, p: usize) { self.storage.offset_col(p) }

	pub fn shift_col_to<S: Storage<T>>(&mut self, s: &S, pos: usize, length: usize) { self.storage.shift_col_to(s, pos, length)}
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

impl<'a, T, RS, C, CS> PtrStorageMut<'a, T, Dynamic, RS, C, CS>
	where T: Element, RS: Dim, C: Dim, CS: Dim
{
	#[inline]
	pub fn offset_row(&mut self, p: usize) { self.storage.offset_row(p) }

	pub fn shift_row_to<S: StorageMut<T>>(&mut self, s: &S, pos: usize, length: usize) { self.storage.shift_row_to(s, pos, length)}
}

impl<'a, T, R, RS, CS> PtrStorageMut<'a, T, R, RS, Dynamic, CS>
	where T: Element, RS: Dim, R: Dim, CS: Dim
{
	#[inline]
	pub fn offset_col(&mut self, p: usize) { self.storage.offset_col(p) }

	pub fn shift_col_to<S: StorageMut<T>>(&mut self, s: &S, pos: usize, length: usize) { self.storage.shift_col_to(s, pos, length)}
}


impl<'a, T, R, RS, C, CS> Into<SliceMut<'a, T, R, RS, C, CS>> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim {
	fn into(self) -> SliceMut<'a, T, R, RS, C, CS> { SliceBase::new(self).into() }
}

impl<'a, T, R, RS, C, CS> InplaceMap<T> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace(f) }
}

impl<'a, T, R, RS, C, CS> InplaceMapOrdered<T> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn map_inplace_ordered<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace_ordered(f) }
}

impl<'a, T, R, RS, C, CS> IndexMut<usize> for PtrStorageMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		let r = index / self.cols();
		let c = index % self.cols();
		self.get_mut(r, c)
	}
}