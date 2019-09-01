use crate::format::*;
use crate::storage::{Storage, StorageMut};
use std::marker::PhantomData;
use crate::{SplittableIterator, Parallel};
use rayon::iter::{IntoParallelIterator};


#[derive(Debug, new)]
pub(crate) struct AxisIterRaw<T, S: Dim> {
	pub(super) length: usize,
	pub(super) stride: S,
	cursor: usize,
	pub(super) ptr: *mut T,
}

unsafe impl<T, S: Dim> Send for AxisIterRaw<T, S> {}

unsafe impl<T, S: Dim> Sync for AxisIterRaw<T, S> {}

impl<T, S: Dim> SplittableIterator for AxisIterRaw<T, S> {
	fn split_at(self, pos: usize) -> (Self, Self) {
		assert!(pos <= self.length);
		let left_ptr = unsafe { self.offset(self.cursor) };
		let right_ptr = if pos != self.length {
			unsafe { self.offset(pos) }
		} else {
			self.ptr
		};

		let left = AxisIterRaw {
			length: pos,
			stride: self.stride,
			cursor: 0,
			ptr: left_ptr,
		};
		let right = AxisIterRaw {
			length: self.length - pos,
			stride: self.stride,
			cursor: 0,
			ptr: right_ptr
		};

		(left, right)
	}
}

impl<T, S: Dim> AxisIterRaw<T, S> {
	unsafe fn offset(&self, pos: usize) -> *mut T {
		debug_assert!(
			pos <= self.length,
			"pos={}, length={:#?}, stride={:#?}",
			pos,
			self.length,
			self.stride
		);
		self.ptr.offset((pos * self.stride.value()) as isize)
	}

	pub(super) fn from_storage<ST, A>(s: &ST, _a: A, pos: usize) -> Self
		where T: Element, ST: Storage<T>, A: Axis<ST::Rows, ST::Cols> + Axis<ST::RowStride, ST::ColStride>,
		      AxisParallel<A, ST::RowStride, ST::ColStride>: Axis<ST::RowStride, ST::ColStride, RetType=S>
	{
		Self::new(
			s.get_axis_size::<AxisParallel<A, ST::Rows, ST::Cols>>().value(),
			s.get_axis_stride::<AxisParallel<A, ST::RowStride, ST::ColStride>>(),
			0,
			<A as Axis<ST::Rows, ST::Cols>>::get_val(s.as_row_ptr(pos), s.as_col_ptr(pos)) as *mut T,
		)
	}
}

impl<T, S: Dim> ExactSizeIterator for AxisIterRaw<T, S> {}

impl<T, S: Dim> Iterator for AxisIterRaw<T, S> {
	type Item = *mut T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cursor >= self.length {
			None
		} else {
			let ptr = unsafe { self.offset(self.cursor) };
			self.cursor += 1;
			Some(ptr)
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.length - self.cursor;
		(len, Some(len))
	}
}

impl<T, S: Dim> DoubleEndedIterator for AxisIterRaw<T, S> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.cursor >= self.length {
			None
		} else {
			self.length -= 1;
			let ptr = unsafe { self.offset(self.length) };
			Some(ptr)
		}
	}
}

/// Ref iterator
#[derive(Debug)]
pub struct AxisIter<'a, T, S: Dim> {
	pub(crate) iter: AxisIterRaw<T, S>,
	_phantoms: PhantomData<&'a T>
}

impl<'a, T, S: Dim> AxisIter<'a, T, S> {
	pub fn new(data: *const T, stride: S, length: usize) -> Self {
		AxisIter {
			iter: AxisIterRaw {
				length,
				stride,
				cursor: 0,
				ptr: data as *mut T
			},
			_phantoms: PhantomData
		}
	}

	pub fn from_storage<ST, A>(s: &ST, _a: A, pos: usize) -> Self
		where T: Element, ST: Storage<T>, A: Axis<ST::Rows, ST::Cols> + Axis<ST::RowStride, ST::ColStride>,
				AxisParallel<A, ST::RowStride, ST::ColStride>: Axis<ST::RowStride, ST::ColStride, RetType=S>
	{
		Self::new(
			<A as Axis<ST::Rows, ST::Cols>>::get_val(s.as_row_ptr(pos), s.as_col_ptr(pos)),
			s.get_axis_stride::<AxisParallel<A, ST::RowStride, ST::ColStride>>(),
			s.get_axis_size::<AxisParallel<A, ST::Rows, ST::Cols>>().value()
		)
	}
}

impl<'a, T, S: Dim> SplittableIterator for AxisIter<'a, T, S> {
	fn split_at(self, pos: usize) -> (Self, Self) {
		let (left, right) = self.iter.split_at(pos);
		(
			AxisIter { iter: left, _phantoms: PhantomData },
			AxisIter { iter: right, _phantoms: PhantomData },
		)
	}
}

impl<'a, T, S: Dim> ExactSizeIterator for AxisIter<'a, T, S> {}

impl<'a, T, S: Dim> Iterator for AxisIter<'a, T, S> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|v| unsafe { &*v })
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<'a, T, S: Dim> DoubleEndedIterator for AxisIter<'a, T, S> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|v| unsafe { &*v })
	}
}

impl<'a, T: Send + Sync, S: Dim> IntoParallelIterator for AxisIter<'a, T, S> {
	type Iter = Parallel<Self>;
	type Item = <Self as Iterator>::Item;

	fn into_par_iter(self) -> Self::Iter { Parallel::new(self) }
}

/// Ref mut iterator
#[derive(Debug)]
pub struct AxisIterMut<'a, T, S: Dim> {
	pub(crate) iter: AxisIterRaw<T, S>,
	_phantoms: PhantomData<&'a T>
}

impl<'a, T, S: Dim> AxisIterMut<'a, T, S> {
	pub fn new(data: *mut T, stride: S, length: usize) -> Self {
		AxisIterMut {
			iter: AxisIterRaw {
				length,
				stride,
				cursor: 0,
				ptr: data
			},
			_phantoms: PhantomData
		}
	}

	pub fn from_storage<ST, A>(s: &mut ST, _a: A, pos: usize) -> Self
		where T: Element, ST: StorageMut<T>, A: Axis<ST::Rows, ST::Cols> + Axis<ST::RowStride, ST::ColStride>,
		      AxisParallel<A, ST::RowStride, ST::ColStride>: Axis<ST::RowStride, ST::ColStride, RetType=S>
	{
		Self::new(
			<A as Axis<ST::Rows, ST::Cols>>::get_val(s.as_row_ptr_mut(pos), s.as_col_ptr_mut(pos)),
			s.get_axis_stride::<AxisParallel<A, ST::RowStride, ST::ColStride>>(),
			s.get_axis_size::<AxisParallel<A, ST::Rows, ST::Cols>>().value()
		)
	}
}

impl<'a, T, S: Dim> SplittableIterator for AxisIterMut<'a, T, S> {
	fn split_at(self, pos: usize) -> (Self, Self) {
		let (left, right) = self.iter.split_at(pos);
		(
			AxisIterMut { iter: left, _phantoms: PhantomData },
			AxisIterMut { iter: right, _phantoms: PhantomData },
		)
	}
}

impl<'a, T, S: Dim> ExactSizeIterator for AxisIterMut<'a, T, S> {}

impl<'a, T, S: Dim> Iterator for AxisIterMut<'a, T, S> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|v| unsafe { &mut *v })
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<'a, T, S: Dim> DoubleEndedIterator for AxisIterMut<'a, T, S> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|v| unsafe { &mut *v })
	}
}

impl<'a, T: Send + Sync, S: Dim> IntoParallelIterator for AxisIterMut<'a, T, S> {
	type Iter = Parallel<Self>;
	type Item = <Self as Iterator>::Item;

	fn into_par_iter(self) -> Self::Iter { Parallel::new(self) }
}


pub fn row_iter<T: Element, S: Storage<T>>(s: &S, pos: usize) -> AxisIter<T, S::ColStride>
{
	AxisIter::from_storage(s, RowAxis, pos)
}

pub fn row_iter_mut<T, S>(s: &mut S, pos: usize) -> AxisIterMut<T, S::ColStride>
	where T: Element, S: StorageMut<T>
{
	AxisIterMut::new(s.as_row_ptr_mut(pos), s.col_stride_dim(), s.cols())
}

pub fn col_iter<T, S>(s: &S, pos: usize) -> AxisIter<T, S::RowStride>
	where T: Element, S: Storage<T>
{
	AxisIter::from_storage(s, ColAxis, pos)
}

pub fn col_iter_mut<T, S>(s: &mut S, pos: usize) -> AxisIterMut<T, S::RowStride>
	where T: Element, S: StorageMut<T>
{
	AxisIterMut::new(s.as_col_ptr_mut(pos), s.row_stride_dim(), s.rows())
}