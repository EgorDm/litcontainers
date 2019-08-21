use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use crate::Wrapper;


/// Slice containing references to scalar values.
pub type Slice<'a, T, R, RS, C, CS> = Wrapper<T, SliceBase<'a, T, PtrStorage<'a, T, R, RS, C, CS>>>;
/// Slice containing mutable references to scalar values.
pub type SliceMut<'a, T, R, RS, C, CS> = Wrapper<T, SliceBase<'a, T, PtrStorageMut<'a, T, R, RS, C, CS>>>;

/// Container containing references to scalar values.
#[repr(C)]
#[derive(Debug, Storage, StorageSize, Strided, Ownable, new)]
pub struct SliceBase<'a, T, S>
	where T: Scalar, S: Storage<T>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(&'a (), T, S)>
}

impl<'a, T, S> StorageMut<T> for SliceBase<'a, T, S>
	where T: Scalar, S: StorageMut<T>
{
	fn as_ptr_mut(&mut self) -> *mut T { self.storage.as_ptr_mut() }
}