use crate::format::*;
use crate::storage::{Storage, StorageMut};
use std::marker::PhantomData;


#[derive(Debug, new)]
pub(super) struct AxisIterRaw<T, S: Dim> {
	pub(super) length: usize,
	pub(super) stride: S,
	cursor: usize,
	ptr: *mut T,
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
	iter: AxisIterRaw<T, S>,
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

/// Ref mut iterator
#[derive(Debug)]
pub struct AxisIterMut<'a, T, S: Dim> {
	iter: AxisIterRaw<T, S>,
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

pub fn row_iter<T, R, C, S>(s: &S, pos: usize) -> AxisIter<T, S::CStride>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	AxisIter::new(s.as_row_ptr(pos), s.col_stride_dim(), s.col_count())
}

pub fn row_iter_mut<T, R, C, S>(s: &mut S, pos: usize) -> AxisIterMut<T, S::CStride>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	AxisIterMut::new(s.as_row_mut_ptr(pos), s.col_stride_dim(), s.col_count())
}

pub fn col_iter<T, R, C, S>(s: &S, pos: usize) -> AxisIter<T, S::RStride>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	AxisIter::new(s.as_col_ptr(pos), s.row_stride_dim(), s.row_count())
}

pub fn col_iter_mut<T, R, C, S>(s: &mut S, pos: usize) -> AxisIterMut<T, S::RStride>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	AxisIterMut::new(s.as_col_mut_ptr(pos), s.row_stride_dim(), s.row_count())
}


#[test]
fn test_axis_iter() {
	let data: Vec<i32> = (0..10).collect();
	let target: Vec<i32> = (0..5).map(|v| v * 2).collect();
	let iter = AxisIter::new(data.as_ptr() as *mut i32, U2, 5);
	let result: Vec<i32> = iter.cloned().collect();
	assert_eq!(result, target);
}