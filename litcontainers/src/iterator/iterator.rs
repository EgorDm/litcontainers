use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use std::mem;

// TODO: implement double ended iterator?

macro_rules! iter_ptr_impl {
	(
		struct $Name: ident : $StorageType: ident as $StorageRef: ty {
			$ptr_fn: ident -> $ElementPtr: ty as $ElementRet: ty,
			primary: $prim_size_fn: ident, $span_fn: ident,
			secondary: $scnd_size_fn: ident, $scnd_stride_fn: ident
		} // TODO: use unsafe $ptr_fn which is safe since we stay within the bounds
	) => {
		pub struct $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			storage: $StorageRef,
			ptr: $ElementPtr,
			ptr_end: $ElementPtr,
			cursor: usize,
			cursor_end: usize,
			_phantoms: PhantomData<(&'a (), R, C)>
		}

		impl<'a, T, R, C, S> $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			pub fn new(storage: $StorageRef) -> Self {
				let size = storage.$prim_size_fn();
				Self::from_range(storage, 0, size)
			}

			pub fn from_range(storage: $StorageRef, cursor: usize, cursor_end: usize) -> Self {
				let ptr = storage.$ptr_fn(cursor);
				let ptr_end = unsafe { ptr.offset(storage.$span_fn(cursor) as isize) };
				Self {
					storage,
					ptr,
					ptr_end,
					cursor,
					cursor_end,
					_phantoms: PhantomData
				}
			}
		}

		impl<'a, T, R, C, S> Iterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			type Item = $ElementRet;

			#[inline]
			fn next(&mut self) -> Option<Self::Item> {
				if self.ptr < self.ptr_end {
					let old = self.ptr;
					unsafe {
						self.ptr = self.ptr.offset(self.storage.$scnd_stride_fn() as isize);
						Some(mem::transmute(old))
					}
				} else if self.cursor < self.cursor_end - 1 {
					self.cursor += 1;
					self.ptr = self.storage.$ptr_fn(self.cursor);
					let size = self.storage.$span_fn(self.cursor);
					self.ptr_end = unsafe { self.ptr.offset(size as isize)};
					self.next()
				} else {
					None
				}
			}

			#[inline]
			fn count(self) -> usize {
				self.size_hint().0
			}

			#[inline]
			fn size_hint(&self) -> (usize, Option<usize>) {
				let line_pos = (((self.ptr_end as usize - self.ptr as usize) / std::mem::size_of::<T>()) as f32 / self.storage.$scnd_stride_fn() as f32).ceil() as usize;
				let size = (self.cursor_end - self.cursor) * self.storage.$scnd_size_fn() - (self.storage.$scnd_size_fn() - line_pos);
				(size, Some(size))
			}
		}

		impl<'a, T, R, C, S> ExactSizeIterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{}
	}
}

iter_ptr_impl! {
	struct RowIterPtr : Storage as &'a S {
		as_row_ptr -> *const T as &'a T,
		primary: row_count, row_index_span,
		secondary: col_count, col_stride
	}
}

iter_ptr_impl! {
	struct RowIterMutPtr : StorageMut as &'a mut S {
		as_row_mut_ptr -> *mut T as &'a mut T,
		primary: row_count, row_index_span,
		secondary: col_count, col_stride
	}
}

iter_ptr_impl! {
	struct ColIterPtr : Storage as &'a S {
		as_col_ptr -> *const T as &'a T,
		primary: col_count, col_index_span,
		secondary: row_count, row_stride
	}
}

iter_ptr_impl! {
	struct ColIterMutPtr : StorageMut as &'a mut S {
		as_col_mut_ptr -> *mut T as &'a mut T,
		primary: col_count, col_index_span,
		secondary: row_count, row_stride
	}
}

macro_rules! iter_ptr_impl_owned {
	(
		struct $Name: ident : $StorageType: ident as $StorageRef: ty {
			$ptr_fn: ident -> $ElementPtr: ty as $ElementRet: ty,
			primary: $prim_size_fn: ident, $span_fn: ident,
			secondary: $scnd_size_fn: ident, $scnd_stride_fn: ident
		} // TODO: use unsafe $ptr_fn which is safe since we stay within the bounds
	) => {
		pub struct $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			storage: $StorageRef,
			ptr: $ElementPtr,
			ptr_end: $ElementPtr,
			cursor: usize,
			cursor_end: usize,
			_phantoms: PhantomData<(&'a (), R, C)>
		}

		impl<'a, T, R, C, S> $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			pub fn new(storage: $StorageRef) -> Self {
				let size = storage.$prim_size_fn();
				Self::from_range(storage, 0, size)
			}

			pub fn from_range(storage: $StorageRef, cursor: usize, cursor_end: usize) -> Self {
				let ptr = storage.$ptr_fn(cursor);
				let ptr_end = unsafe { ptr.offset(storage.$span_fn(cursor) as isize) };
				Self {
					storage,
					ptr,
					ptr_end,
					cursor,
					cursor_end,
					_phantoms: PhantomData
				}
			}
		}

		impl<'a, T, R, C, S> Iterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{
			type Item = $ElementRet;

			#[inline]
			fn next(&mut self) -> Option<Self::Item> {
				if self.ptr < self.ptr_end {
					let old = self.ptr;
					unsafe {
						self.ptr = self.ptr.offset(self.storage.$scnd_stride_fn() as isize);
						Some(*old)
					}
				} else if self.cursor + 1 < self.cursor_end {
					self.cursor += 1;
					self.ptr = self.storage.$ptr_fn(self.cursor);
					let size = self.storage.$span_fn(self.cursor);
					self.ptr_end = unsafe { self.ptr.offset(size as isize)};
					self.next()
				} else {
					None
				}
			}

			#[inline]
			fn count(self) -> usize {
				self.size_hint().0
			}

			#[inline]
			fn size_hint(&self) -> (usize, Option<usize>) {
				let line_pos = (((self.ptr_end as isize - self.ptr as isize).max(0) as usize
					/ std::mem::size_of::<T>()) as f32 / self.storage.$scnd_stride_fn() as f32).ceil() as usize;
				let size = (self.cursor_end - self.cursor) * self.storage.$scnd_size_fn() - (self.storage.$scnd_size_fn() - line_pos);
				(size, Some(size))
			}
		}

		impl<'a, T, R, C, S> ExactSizeIterator for $Name<'a, T, R, C, S>
			where T: Scalar + 'a, R: Dim, C: Dim, S: $StorageType<T, R, C>
		{}
	}
}

iter_ptr_impl_owned! {
	struct RowIterPtrOwned : Storage as S {
		as_row_ptr -> *const T as T,
		primary: row_count, row_index_span,
		secondary: col_count, col_stride
	}
}

iter_ptr_impl_owned! {
	struct ColIterPtrOwned : Storage as S {
		as_col_ptr -> *const T as T,
		primary: col_count, col_index_span,
		secondary: row_count, row_stride
	}
}