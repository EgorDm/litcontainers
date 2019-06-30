use crate::format::*;
use crate::storage::*;

pub trait DynamicRowStorage<T, C>: StorageMut<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	// TODO: add amortized version of resize otherwise -> super slow growing
	#[inline]
	unsafe fn set_row_count(&mut self, count: usize);
}

pub trait DynamicColStorage<T, R>: StorageMut<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	#[inline]
	unsafe fn set_col_count(&mut self, count: usize);
}

