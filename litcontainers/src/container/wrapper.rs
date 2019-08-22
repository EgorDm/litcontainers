use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use std::fmt;

// Container storing scalar values. Wraps around given storage.
#[derive(Debug, Storage, StorageSize, Strided, Ownable, new)]
pub struct Container<T, S>
	where T: Element, S: Storage<T>
{
	storage: S,
	_phantoms: PhantomData<(T)>,
}

impl<T, S> StorageMut<T> for Container<T, S>
	where T: Element, S: StorageMut<T>
{
	fn as_ptr_mut(&mut self) -> *mut T { self.storage.as_ptr_mut() }
}

impl<T, S> StorageConstructor<T> for Container<T, S>
	where T: Element, S: Storage<T> + StorageConstructor<T>
{
	fn from_value(s: Size<Self::Rows, Self::Cols>, value: T) -> Self { S::from_value(s, value).into()  }
}

impl<T, S> DynamicRowStorage<T> for Container<T, S>
	where T: Element, S: StorageMut<T> + DynamicRowStorage<T>
{
	fn set_rows(&mut self, count: usize) {
		self.storage.set_rows(count)
	}
}

impl<T, S> DynamicColStorage<T> for Container<T, S>
	where T: Element, S: StorageMut<T> + DynamicColStorage<T>
{
	fn set_cols(&mut self, count: usize) { self.storage.set_cols(count) }
}

impl<T, S> fmt::Display for Container<T, S>
	where T: Element, S: StorageMut<T>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", Fmt(|f| print_storage(self, f)))
	}
}

impl<T: Element, S: Storage<T>> From<S> for Container<T, S> {
	fn from(s: S) -> Self { Container::new(s) }
}