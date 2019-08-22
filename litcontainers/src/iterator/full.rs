use crate::format::*;
use super::axis::AxisIterRaw;
use std::marker::PhantomData;
use crate::{Storage, StorageMut, SliceRange};

#[derive(Debug)]
pub struct FullIterCore<T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	length: P,
	cursor: usize,
	stride: PS,
	axis: AxisIterRaw<T, SS>,
	ptr: *mut T,
}

// todo: CREATE A WARPPER which can do into iter
impl<T, P, PS, SS> FullIterCore<T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	pub(super) fn new<S: Dim>(ptr: *mut T, prim_size: P, prim_stride: PS, scnd_size: S, scnd_stride: SS) -> Self {
		Self {
			length: prim_size,
			cursor: 1,
			stride: prim_stride,
			axis: AxisIterRaw::new(scnd_size.value(), scnd_stride, 0, ptr),
			ptr
		}
	}

	unsafe fn offset(&self, pos: usize) -> *mut T {
		debug_assert!(
			pos <= self.length.value(),
			"pos={}, length={:#?}, stride={:#?}",
			pos,
			self.length,
			self.stride
		);
		self.ptr.offset((pos * self.stride.value()) as isize)
	}
}

impl<T, P, PS, SS> ExactSizeIterator for FullIterCore<T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim {}

impl<T, P, PS, SS> Iterator for FullIterCore<T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	type Item = *mut T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.axis.len() > 0 {
			self.axis.next()
		} else if self.cursor < self.length.value() {
			self.axis = AxisIterRaw::new(
				self.axis.length,
				self.axis.stride,
				0,
				unsafe { self.offset(self.cursor) }
			);
			self.cursor += 1;
			self.next()
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = (self.length.value() - self.cursor) * self.axis.length + self.axis.len();
		(len, Some(len))
	}
}

#[derive(Debug)]
pub struct FullIter<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	iter: FullIterCore<T, P, PS, SS>,
	_phantoms: PhantomData<&'a T>
}

impl<'a, T, P, PS, SS> FullIter<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	pub fn new<S: Dim>(ptr: *const T, prim_size: P, prim_stride: PS, scnd_size: S, scnd_stride: SS) -> Self {
		Self {
			iter: FullIterCore::new(ptr as *mut T, prim_size, prim_stride, scnd_size, scnd_stride),
			_phantoms: PhantomData
		}
	}

	pub fn from_storage<S, A>(s: &S, a: A) -> Self
		where T: Element, S: Storage<T>,
		      A: Axis<S::Rows, S::Cols, RetType=P> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      <A as Axis<S::RowStride, S::ColStride>>::Parallel: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			s.as_ptr(),
			s.get_axis_size::<A>(),
			s.get_axis_stride::<A>(),
			s.get_axis_size::<<A as Axis<S::Rows, S::Cols>>::Parallel>(),
			s.get_axis_stride::<<A as Axis<S::RowStride, S::ColStride>>::Parallel>()
		)
	}

	pub fn from_storage_range<S, A, R>(s: &S, a: A, r: R) -> Self
		where T: Element, S: Storage<T>, R: SliceRange<<A as Axis<S::Rows, S::Cols>>::RetType, Size=P>,
		      A: Axis<S::Rows, S::Cols> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      <A as Axis<S::RowStride, S::ColStride>>::Parallel: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			unsafe {<A as Axis<S::RowStride, S::ColStride>>::get_val(s.as_row_ptr_unchecked(r.begin()), s.as_col_ptr_unchecked(r.begin())) },
			r.size(),
			s.get_axis_stride::<A>(),
			s.get_axis_size::<<A as Axis<S::Rows, S::Cols>>::Parallel>(),
			s.get_axis_stride::<<A as Axis<S::RowStride, S::ColStride>>::Parallel>()
		)
	}
}

impl<'a, T, P, PS, SS> ExactSizeIterator for FullIter<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim {}

impl<'a, T, P, PS, SS> Iterator for FullIter<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|v| unsafe { &*v })
	}

	fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

#[derive(Debug)]
pub struct FullIterMut<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	iter: FullIterCore<T, P, PS, SS>,
	_phantoms: PhantomData<&'a T>
}

impl<'a, T, P, PS, SS> FullIterMut<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	pub fn new<S: Dim>(ptr: *mut T, prim_size: P, prim_stride: PS, scnd_size: S, scnd_stride: SS) -> Self {
		Self {
			iter: FullIterCore::new(ptr, prim_size, prim_stride, scnd_size, scnd_stride),
			_phantoms: PhantomData
		}
	}

	pub fn from_storage<S, A>(s: &mut S, a: A) -> Self
		where T: Element, S: StorageMut<T>,
		      A: Axis<S::Rows, S::Cols, RetType=P> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      <A as Axis<S::RowStride, S::ColStride>>::Parallel: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			s.as_ptr_mut(),
			s.get_axis_size::<A>(),
			s.get_axis_stride::<A>(),
			s.get_axis_size::<<A as Axis<S::Rows, S::Cols>>::Parallel>(),
			s.get_axis_stride::<<A as Axis<S::RowStride, S::ColStride>>::Parallel>()
		)
	}

	pub fn from_storage_range<S, A, R>(s: &mut S, a: A, r: R) -> Self
		where T: Element, S: StorageMut<T>, R: SliceRange<<A as Axis<S::Rows, S::Cols>>::RetType, Size=P>,
		      A: Axis<S::Rows, S::Cols> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      <A as Axis<S::RowStride, S::ColStride>>::Parallel: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			unsafe {<A as Axis<S::RowStride, S::ColStride>>::get_val(s.as_row_ptr_mut_unchecked(r.begin()), s.as_col_ptr_mut_unchecked(r.begin())) },
			r.size(),
			s.get_axis_stride::<A>(),
			s.get_axis_size::<<A as Axis<S::Rows, S::Cols>>::Parallel>(),
			s.get_axis_stride::<<A as Axis<S::RowStride, S::ColStride>>::Parallel>()
		)
	}
}

impl<'a, T, P, PS, SS> ExactSizeIterator for FullIterMut<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim {}

impl<'a, T, P, PS, SS> Iterator for FullIterMut<'a, T, P, PS, SS>
	where P: Dim, PS: Dim, SS: Dim
{
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|v| unsafe { &mut *v })
	}

	fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub type FullRowIter<'a, T, S: StorageSize + Strided> = FullIter<'a, T, S::Rows, S::RowStride, S::ColStride>;
pub type FullRowIterMut<'a, T, S: StorageSize + Strided> = FullIterMut<'a, T, S::Rows, S::RowStride, S::ColStride>;
pub type FullColIter<'a, T, S: StorageSize + Strided> = FullIter<'a, T, S::Cols, S::ColStride, S::RowStride>;
pub type FullColIterMut<'a, T, S: StorageSize + Strided> = FullIterMut<'a, T, S::Cols, S::ColStride, S::RowStride>;

/*#[derive(Debug)]
pub struct FullIterCoreOwned<T, P, PS, SS, S>
	where P: Dim, PS: Dim, SS: Dim, S: Storage<T>
{
	owner: S,

}*/

pub type FullAxisIter<'a, T: Element, S: Storage<T>, A: Axis<S::Rows, S::Cols> + Axis<S::RowStride, S::ColStride>> = FullIter<'a, T, <A as Axis<S::Rows, S::Cols>>::RetType, <A as Axis<S::RowStride, S::ColStride>>::RetType, <<A as Axis<S::RowStride, S::ColStride>>::Parallel as Axis<S::RowStride, S::ColStride>>::RetType>;

pub fn full_row_iter_test<T, S, A>(s: &S, a: A)
	-> FullAxisIter<T, S, A>
	where T: Element, S: Storage<T>, A: Axis<S::Rows, S::Cols> + Axis<S::RowStride, S::ColStride>
{
	FullIter::new(s.as_ptr(), s.get_axis_size::<A>(), s.get_axis_stride::<A>(), s.get_axis_size::<<A as Axis<S::Rows, S::Cols>>::Parallel>(), s.get_axis_stride::<<A as Axis<S::RowStride, S::ColStride>>::Parallel>())
}


#[macro_export]
macro_rules! full_row_iter (
	($s: expr) => {
		FullIter::new($s.as_ptr(), $s.row_dim(), $s.row_stride_dim(), $s.col_dim(), $s.col_stride_dim())
	};
	($s: expr, $r: expr) => {
		FullIter::new($s.as_row_ptr($r.begin()), $r.size(), $s.row_stride_dim(), $s.col_dim(), $s.col_stride_dim())
	};
	(mut $s: expr) => {
		FullIterMut::new($s.as_ptr_mut(), $s.row_dim(), $s.row_stride_dim(), $s.col_dim(), $s.col_stride_dim())
	};
	(mut $s: expr, $r: expr) => {
		FullIterMut::new($s.as_row_ptr_mut($r.begin()), $r.size(), $s.row_stride_dim(), $s.col_dim(), $s.col_stride_dim())
	};
);

#[macro_export]
macro_rules! full_col_iter (
	($s: expr) => {
		FullIter::new($s.as_ptr(), $s.col_dim(), $s.col_stride_dim(), $s.row_dim(), $s.row_stride_dim())
	};
	($s: expr, $r: expr) => {
		FullIter::new($s.as_col_ptr($r.begin()), $r.size(), $s.col_stride_dim(), $s.row_dim(), $s.row_stride_dim())
	};
	(mut $s: expr) => {
		FullIterMut::new($s.as_ptr_mut(), $s.col_dim(), $s.col_stride_dim(), $s.row_dim(), $s.row_stride_dim())
	};
	(mut $s: expr, $r: expr) => {
		FullIterMut::new($s.as_col_ptr_mut($r.begin()), $r.size(), $s.col_stride_dim(), $s.row_dim(), $s.row_stride_dim())
	};
);