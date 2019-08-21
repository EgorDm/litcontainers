use crate::format::*;
use crate::storage::*;

pub trait DynamicRowStorage<T: Scalar>: StorageMut<T> + StorageSize<Rows=Dynamic> {
	// TODO: add amortized version of resize otherwise -> super slow growing
	#[inline]
	fn set_rows(&mut self, count: usize);
}

pub trait DynamicColStorage<T: Scalar>: StorageMut<T> + StorageSize<Cols=Dynamic> {
	#[inline]
	fn set_cols(&mut self, count: usize);
}

