use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(T, R, C, S)>
}

impl<T, R, C, S> SizedStorage<R, C> for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	fn row_dim(&self) -> R { self.storage.row_dim() }

	fn col_dim(&self) -> C { self.storage.col_dim() }
}

impl<T, R, C, S> Storage<T, R, C> for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	type RStride = S::RStride;
	type CStride = S::CStride;

	fn row_stride_dim(&self) -> Self::RStride { self.storage.row_stride_dim() }

	fn col_stride_dim(&self) -> Self::CStride { self.storage.col_stride_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.storage.get_index_ptr_unchecked(i) }
}

impl<T, R, C, S> StorageMut<T, R, C> for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self. storage.get_index_mut_ptr_unchecked(i) }
}

impl<T, R, C, S> Ownable<T, R, C> for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
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

impl<T, R, C, S> StorageConstructor<T, R, C> for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C> + StorageConstructor<T, R, C>
{
	fn from_value(rows: R, cols: C, value: T) -> Self {
		Container {
			storage: S::from_value(rows, cols, value),
			_phantoms: PhantomData
		}
	}
}
