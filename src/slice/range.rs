use crate::format::dimensions::{Dim, U1, Dynamic};
use std::ops::Range;

pub trait SliceRange<D: Dim> {
	type Size: Dim;

	fn begin(&self) -> usize;

	fn end(&self) -> usize;

	fn size(&self) -> Self::Size;
}

impl<D: Dim> SliceRange<D> for usize {
	type Size = U1;

	#[inline(always)]
	fn begin(&self) -> usize {
		*self
	}

	#[inline(always)]
	fn end(&self) -> usize {
		*self + 1
	}

	#[inline(always)]
	fn size(&self) -> Self::Size {
		U1
	}
}

impl<D: Dim> SliceRange<D> for Range<usize> {
	type Size = Dynamic;

	#[inline(always)]
	fn begin(&self) -> usize {
		self.start
	}

	#[inline(always)]
	fn end(&self) -> usize {
		self.end
	}

	#[inline(always)]
	fn size(&self) -> Self::Size {
		Dynamic::new(self.end - self.start)
	}
}