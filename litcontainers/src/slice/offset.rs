use crate::{Element, Storage, StorageSize, Dynamic, Axis, StorageMut};

pub trait OffsetStorage<T: Element>: StorageSize

{
	fn offset<A>(&mut self, p: usize)
		where A: Axis<Self::Rows, Self::Cols, RetType=Dynamic>;

	fn shift_to<A, S>(&mut self, s: &S, pos: usize, length: usize)
		where S: Storage<T>, A: Axis<Self::Rows, Self::Cols, RetType=Dynamic>;
}

pub trait OffsetStorageMut<T: Element>: StorageSize {
	fn offset_mut<A>(&mut self, p: usize)
		where A: Axis<Self::Rows, Self::Cols, RetType=Dynamic>;

	fn shift_to_mut<A, S>(&mut self, s: &S, pos: usize, length: usize)
		where S: StorageMut<T>, A: Axis<Self::Rows, Self::Cols, RetType=Dynamic>;
}