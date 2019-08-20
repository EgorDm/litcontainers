use crate::format::*;
use crate::storage::{Storage, StorageMut};

pub trait RowVecStorage<T: Scalar>: Storage<T> + StorageSize<Rows=U1> {}

impl<T, S> RowVecStorage<T> for S
	where T: Scalar, S: Storage<T> + StorageSize<Rows=U1> {}

pub trait RowVecStorageMut<T: Scalar>: StorageMut<T> + StorageSize<Rows=U1> {}

impl<T, S> RowVecStorageMut<T> for S
	where T: Scalar, S: StorageMut<T> + StorageSize<Rows=U1> {}


pub trait ColVecStorage<T: Scalar>: Storage<T> + StorageSize<Cols=U1> {}

impl<T, S> ColVecStorage<T> for S
	where T: Scalar, S: Storage<T> + StorageSize<Cols=U1> {}

pub trait ColVecStorageMut<T: Scalar>: StorageMut<T> + StorageSize<Cols=U1> {}

impl<T, S> ColVecStorageMut<T> for S
	where T: Scalar, S: StorageMut<T> + StorageSize<Cols=U1> {}