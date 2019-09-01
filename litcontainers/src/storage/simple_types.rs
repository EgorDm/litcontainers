use crate::format::*;
use crate::storage::{Storage, StorageMut};
use crate::iterator::*;
use rayon::iter::IntoParallelIterator;

pub trait RowVecStorage<T: Element>: Storage<T> + StorageSize<Rows=U1> {
	fn par_iter(& self) -> Parallel<AxisIter<T, Self::ColStride>> {
		row_iter(self, 0).into_par_iter()
	}
}

impl<T, S> RowVecStorage<T> for S
	where T: Element, S: Storage<T> + StorageSize<Rows=U1> {}

pub trait RowVecStorageMut<T: Element>: StorageMut<T> + StorageSize<Rows=U1> {
	fn par_iter_mut(&mut self) -> Parallel<AxisIterMut<T, Self::ColStride>> {
		row_iter_mut(self, 0).into_par_iter()
	}
}

impl<T, S> RowVecStorageMut<T> for S
	where T: Element, S: StorageMut<T> + StorageSize<Rows=U1> {}


pub trait ColVecStorage<T: Element>: Storage<T> + StorageSize<Cols=U1> {
	fn par_iter(& self) -> Parallel<AxisIter<T, Self::RowStride>> {
		col_iter(self, 0).into_par_iter()
	}
}

impl<T, S> ColVecStorage<T> for S
	where T: Element, S: Storage<T> + StorageSize<Cols=U1> {}

pub trait ColVecStorageMut<T: Element>: StorageMut<T> + StorageSize<Cols=U1> {
	fn par_iter_mut(&mut self) -> Parallel<AxisIterMut<T, Self::RowStride>> {
		col_iter_mut(self, 0).into_par_iter()
	}
}

impl<T, S> ColVecStorageMut<T> for S
	where T: Element, S: StorageMut<T> + StorageSize<Cols=U1> {}