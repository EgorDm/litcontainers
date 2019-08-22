use crate::format::*;
use crate::storage::{Storage, StorageMut};

pub trait RowVecStorage<T: Element>: Storage<T> + StorageSize<Rows=U1> {}

impl<T, S> RowVecStorage<T> for S
	where T: Element, S: Storage<T> + StorageSize<Rows=U1> {}

pub trait RowVecStorageMut<T: Element>: StorageMut<T> + StorageSize<Rows=U1> {}

impl<T, S> RowVecStorageMut<T> for S
	where T: Element, S: StorageMut<T> + StorageSize<Rows=U1> {}


pub trait ColVecStorage<T: Element>: Storage<T> + StorageSize<Cols=U1> {}

impl<T, S> ColVecStorage<T> for S
	where T: Element, S: Storage<T> + StorageSize<Cols=U1> {}

pub trait ColVecStorageMut<T: Element>: StorageMut<T> + StorageSize<Cols=U1> {}

impl<T, S> ColVecStorageMut<T> for S
	where T: Element, S: StorageMut<T> + StorageSize<Cols=U1> {}