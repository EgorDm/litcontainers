use crate::format::dimensions::{Dim, U1, Dynamic};
use std::ops::Range;

pub trait SliceRange {
	type Size: Dim;

	fn begin(&self) -> usize;

	fn end(&self) -> usize;

	fn size(&self) -> Self::Size;
}

impl SliceRange for usize {
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

impl SliceRange for Range<usize> {
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

pub struct SizedRange<D: Dim> {
	start: usize,
	size: D
}

impl<D: Dim> SizedRange<D> {
	pub fn new(start: usize, size: D) -> Self { Self {start, size} }
}

impl<D: Dim> SliceRange for SizedRange<D> {
	type Size = D;

	fn begin(&self) -> usize { self.start }

	fn end(&self) -> usize { self.start + self.size.value() }

	fn size(&self) -> Self::Size { self.size }
}