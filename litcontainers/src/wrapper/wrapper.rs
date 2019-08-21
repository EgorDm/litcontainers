use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use std::fmt;

#[derive(Debug, Storage, StorageSize, Strided, Ownable, new)]
pub struct Wrapper<T, S>
	where T: Scalar, S: Storage<T>
{
	storage: S,
	_phantoms: PhantomData<(T)>,
}

impl<T, S> StorageMut<T> for Wrapper<T, S>
	where T: Scalar, S: StorageMut<T>
{
	fn as_ptr_mut(&mut self) -> *mut T { self.storage.as_ptr_mut() }
}

impl<T, S> StorageConstructor<T> for Wrapper<T, S>
	where T: Scalar, S: Storage<T> + StorageConstructor<T>
{
	fn from_value(s: Size<Self::Rows, Self::Cols>, value: T) -> Self { S::from_value(s, value).into()  }
}

impl<T, S> DynamicRowStorage<T> for Wrapper<T, S>
	where T: Scalar, S: StorageMut<T> + DynamicRowStorage<T>
{
	fn set_rows(&mut self, count: usize) {
		self.storage.set_rows(count)
	}
}

impl<T, S> DynamicColStorage<T> for Wrapper<T, S>
	where T: Scalar, S: StorageMut<T> + DynamicColStorage<T>
{
	fn set_cols(&mut self, count: usize) { self.storage.set_cols(count) }
}

impl<T, S> fmt::Display for Wrapper<T, S>
	where T: Scalar, S: StorageMut<T>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}

impl<T: Scalar, S: Storage<T>> From<S> for Wrapper<T, S> {
	fn from(s: S) -> Self { Wrapper::new(s) }
}