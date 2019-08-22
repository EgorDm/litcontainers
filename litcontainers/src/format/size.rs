use super::Dim;
use std::fmt;
use crate::{Fmt, Axis};

pub type SSize<S: StorageSize> = Size<S::Rows, S::Cols>;

// TODO: Can we write it more generic for possible 3d sizes?
#[derive(Debug, PartialEq, Eq, Clone, new)]
pub struct Size<R: Dim, C: Dim> {
	pub rows: R,
	pub cols: C,
}

impl<R: Dim, C: Dim> StorageSize for Size<R, C> {
	type Rows = R;
	type Cols = C;

	#[inline]
	fn row_dim(&self) -> Self::Rows { self.rows }

	#[inline]
	fn col_dim(&self) -> Self::Cols { self.cols }
}

pub trait StorageSize {
	type Rows: Dim;
	type Cols: Dim;

	#[inline]
	fn rows(&self) -> usize { self.row_dim().value() }

	#[inline]
	fn row_dim(&self) -> Self::Rows;

	#[inline]
	fn cols(&self) -> usize { self.col_dim().value() }

	#[inline]
	fn col_dim(&self) -> Self::Cols;

	#[inline]
	fn len(&self) -> usize { self.rows() * self.cols() }

	#[inline]
	fn size(&self) -> Size<Self::Rows, Self::Cols> { Size::new(self.row_dim(), self.col_dim()) }

	#[inline]
	fn equal_size<OS: StorageSize>(&self, o: &OS) -> bool {
		self.cols() == o.cols() && self.rows() == o.rows()
	}

	#[inline]
	fn get_axis_size<A: Axis<Self::Rows, Self::Cols>>(&self) -> A::RetType { A::get_axis(self.row_dim(), self.col_dim()) }
}

impl<R: Dim, C: Dim> fmt::Display for Size<R, C> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Size(Rows = {}, Cols = {})", Fmt(|f| self.row_dim().pfmt(f)), Fmt(|f| self.col_dim().pfmt(f)))
	}
}
