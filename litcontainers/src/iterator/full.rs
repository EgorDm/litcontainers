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

	unsafe fn reset(&mut self, ptr: *mut T) {
		self.cursor = 1;
		self.ptr = ptr;
		self.axis = AxisIterRaw::new(self.axis.length, self.axis.stride, 0, ptr);
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
	fn new<S: Dim>(ptr: *const T, prim_size: P, prim_stride: PS, scnd_size: S, scnd_stride: SS) -> Self {
		Self {
			iter: FullIterCore::new(ptr as *mut T, prim_size, prim_stride, scnd_size, scnd_stride),
			_phantoms: PhantomData
		}
	}

	pub fn from_storage<S, A>(s: &S, _a: A) -> Self
		where T: Element, S: Storage<T>,
		      A: Axis<S::Rows, S::Cols, RetType=P> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      AxisParallel<A, S::RowStride, S::ColStride>: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			s.as_ptr(),
			s.get_axis_size::<A>(),
			s.get_axis_stride::<A>(),
			s.get_axis_size::<<A as Axis<S::Rows, S::Cols>>::Parallel>(),
			s.get_axis_stride::<<A as Axis<S::RowStride, S::ColStride>>::Parallel>()
		)
	}

	pub fn from_storage_range<S, A, R>(s: &S, _a: A, r: R) -> Self
		where T: Element, S: Storage<T>, R: SliceRange<Size=P>,
		      A: Axis<S::Rows, S::Cols> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      AxisParallel<A, S::RowStride, S::ColStride>: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			unsafe { <A as Axis<S::RowStride, S::ColStride>>::get_val(s.as_row_ptr_unchecked(r.begin()), s.as_col_ptr_unchecked(r.begin())) },
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
	fn new<S: Dim>(ptr: *mut T, prim_size: P, prim_stride: PS, scnd_size: S, scnd_stride: SS) -> Self {
		Self {
			iter: FullIterCore::new(ptr, prim_size, prim_stride, scnd_size, scnd_stride),
			_phantoms: PhantomData
		}
	}

	pub fn from_storage<S, A>(s: &mut S, _a: A) -> Self
		where T: Element, S: StorageMut<T>,
		      A: Axis<S::Rows, S::Cols, RetType=P> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      AxisParallel<A, S::RowStride, S::ColStride>: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			s.as_ptr_mut(),
			s.get_axis_size::<A>(),
			s.get_axis_stride::<A>(),
			s.get_axis_size::<AxisParallel<A, S::Rows, S::Cols>>(),
			s.get_axis_stride::<AxisParallel<A, S::RowStride, S::ColStride>>()
		)
	}

	pub fn from_storage_range<S, A, R>(s: &mut S, _a: A, r: R) -> Self
		where T: Element, S: StorageMut<T>, R: SliceRange<Size=P>,
		      A: Axis<S::Rows, S::Cols> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      <A as Axis<S::RowStride, S::ColStride>>::Parallel: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		Self::new(
			unsafe { <A as Axis<S::RowStride, S::ColStride>>::get_val(s.as_row_ptr_mut_unchecked(r.begin()), s.as_col_ptr_mut_unchecked(r.begin())) },
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

#[derive(Debug)]
pub struct FullIterOwned<T, P, PS, SS, S>
	where T: Element, P: Dim, PS: Dim, SS: Dim, S: Storage<T>
{
	storage: S,
	iter: FullIterCore<T, P, PS, SS>
}

impl<T, P, PS, SS, S> FullIterOwned<T, P, PS, SS, S>
	where T: Element, P: Dim, PS: Dim, SS: Dim, S: Storage<T>
{
	pub fn from_storage<A>(s: S, _a: A) -> Self
		where T: Element,
		      A: Axis<S::Rows, S::Cols, RetType=P> + Axis<S::RowStride, S::ColStride, RetType=PS>,
		      AxisParallel<A, S::RowStride, S::ColStride>: Axis<S::RowStride, S::ColStride, RetType=SS>
	{
		let prim_size = s.get_axis_size::<A>();
		let prim_stride = s.get_axis_stride::<A>();
		let scnd_size = s.get_axis_size::<AxisParallel<A, S::Rows, S::Cols>>();
		let scnd_stride = s.get_axis_stride::<AxisParallel<A, S::RowStride, S::ColStride>>();

		let mut ret = Self {
			storage: s,
			iter: FullIterCore::new(std::ptr::null_mut(), prim_size, prim_stride, scnd_size, scnd_stride)
		};
		unsafe { ret.iter.reset(ret.storage.as_ptr() as *mut T); }
		ret
	}
}

impl<T, P, PS, SS, S> ExactSizeIterator for FullIterOwned<T, P, PS, SS, S>
	where T: Element, P: Dim, PS: Dim, SS: Dim, S: Storage<T>
{}

impl<T, P, PS, SS, S> Iterator for FullIterOwned<T, P, PS, SS, S>
	where T: Element, P: Dim, PS: Dim, SS: Dim, S: Storage<T>
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|v| unsafe { *v })
	}

	fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub type FullAxisIter<'a, T, S, A>
= FullIter< 'a, T,
	AxisRes<A, <S as StorageSize>::Rows, <S as StorageSize>::Cols>,
	AxisRes<A, <S as Strided>::RowStride, <S as Strided>::ColStride>,
	AxisParallelRes<A, <S as Strided>::RowStride, <S as Strided>::ColStride>
>;

pub type FullAxisIterMut<'a, T, S, A>
= FullIterMut<'a, T,
	AxisRes<A, <S as StorageSize>::Rows, <S as StorageSize>::Cols>,
	AxisRes<A, <S as Strided>::RowStride, <S as Strided>::ColStride>,
	AxisParallelRes<A, <S as Strided>::RowStride, <S as Strided>::ColStride>
>;

pub type FullAxisIterOwned<T, S, A>
= FullIterOwned<T,
	AxisRes<A, <S as StorageSize>::Rows, <S as StorageSize>::Cols>,
	AxisRes<A, <S as Strided>::RowStride, <S as Strided>::ColStride>,
	AxisParallelRes<A, <S as Strided>::RowStride, <S as Strided>::ColStride>,
	S
>;