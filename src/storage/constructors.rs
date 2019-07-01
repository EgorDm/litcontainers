use crate::storage::*;
use crate::format::{Dim, Scalar};

pub trait StorageConstructor<T, R, C>: StorageMut<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn from_value(rows: R, cols: C, value: T) -> Self;

	#[inline]
	fn zeros(rows: R, cols: C) -> Self {
		Self::from_value(rows, cols, T::default())
	}
}