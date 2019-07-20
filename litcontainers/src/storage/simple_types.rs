use crate::format::*;
use crate::storage::{Storage, StorageMut};

pub trait RowVecStorage<T: Scalar, C: Dim>: Storage<T, U1, C> {}

impl<T, C, S> RowVecStorage<T, C> for S
	where T: Scalar, C: Dim, S: Storage<T, U1, C> {}

pub trait RowVecStorageMut<T: Scalar, C: Dim>: StorageMut<T, U1, C> + RowVecStorage<T, C> {}

impl<T, C, S> RowVecStorageMut<T, C> for S
	where T: Scalar, C: Dim, S: StorageMut<T, U1, C> {}


pub trait ColVecStorage<T: Scalar, R: Dim>: Storage<T, R, U1> {}

impl<T, R, S> ColVecStorage<T, R> for S
	where T: Scalar, R: Dim, S: Storage<T, R, U1> {}

pub trait ColVecStorageMut<T: Scalar, R: Dim>: StorageMut<T, R, U1> + RowVecStorage<T, R> {}

impl<T, R, S> ColVecStorageMut<T, R> for S
	where T: Scalar, R: Dim, S: StorageMut<T, R, U1> + RowVecStorage<T, R> {}