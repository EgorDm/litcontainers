use crate::format::*;
use crate::storage::*;
use crate::*;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(new)]
pub struct OwnedProvider<T, S>
	where T: Element, S: Storage<T>
{
	data: S,
	_phantoms: PhantomData<(T)>
}

impl<'a, T, S> Operation for OwnedProvider<T, S>
	where T: Element, S: Storage<T>
{
	type Type = T;
	type Rows = S::Rows;
	type Cols = S::Cols;
	type Result = S::OwnedType;

	fn apply(self) -> Self::Result {
		self.data.owned()
	}
}

#[derive(new)]
pub struct BorrowedProvider<'a, T, S>
	where T: Element, S: Storage<T>
{
	data: &'a S,
	_phantoms: PhantomData<(T)>
}

impl<'a, T, S> Operation for BorrowedProvider<'a, T, S>
	where T: Element, S: Storage<T>
{
	type Type = T;
	type Rows = S::Rows;
	type Cols = S::Cols;
	type Result = Slice<'a, T, Self::Rows, S::RowStride, Self::Cols, S::ColStride>;

	fn apply(self) -> Self::Result { self.data.into_slice() }
}