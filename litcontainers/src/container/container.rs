use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use std::fmt::{Display, Formatter, Error};

/// Container storing scalar values in a col major order
pub type ContainerCM<T, R, C> = Container<T, R, C, VecStorageCM<T, R, C>>;
/// Container storing scalar values in a row major order
pub type ContainerRM<T, R, C> = Container<T, R, C, VecStorageRM<T, R, C>>;

// Container storing scalar values. Container is always the owner of its data.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(T, R, C, S)>
}

impl<T, R, C, S> Container<T, R, C, S> where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C> {
	pub fn new(storage: S) -> Self {
		Container {
			storage,
			_phantoms: PhantomData
		}
	}
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
		Self::OwnedType::new(self.storage.owned())
	}

	fn clone_owned(&self) -> Self::OwnedType {
		Self::OwnedType::new(self.storage.clone_owned())
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

impl<T, C, S> DynamicRowStorage<T, C> for Container<T, Dynamic, C, S>
	where T: Scalar, C: Dim, S: StorageMut<T, Dynamic, C> + DynamicRowStorage<T, C>
{
	fn set_row_count(&mut self, count: usize) {
		self.storage.set_row_count(count)
	}
}

impl<T, R, S> DynamicColStorage<T, R> for Container<T, R, Dynamic, S>
	where T: Scalar, R: Dim, S: StorageMut<T, R, Dynamic> + DynamicColStorage<T, R>
{
	fn set_col_count(&mut self, count: usize) {
		self.storage.set_col_count(count)
	}
}

impl<T, R, C, S> Display for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C> + StorageConstructor<T, R, C>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}