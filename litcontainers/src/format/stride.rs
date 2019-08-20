use super::Dim;

pub type SStrides<S: Strided> = Strides<S::RowStride, S::ColStride>;

#[derive(Debug, PartialEq, Eq, Clone, new)]
pub struct Strides<RS, CS> {
	row_stride: RS,
	col_stride: CS,
}

impl<RS: Dim, CS: Dim> Strided for Strides<RS, CS> {
	type RowStride = RS;
	type ColStride = CS;

	#[inline]
	fn row_stride_dim(&self) -> Self::RowStride { self.row_stride }

	#[inline]
	fn col_stride_dim(&self) -> Self::ColStride { self.col_stride }
}

pub trait Strided {
	type RowStride: Dim;
	type ColStride: Dim;

	#[inline]
	fn row_stride(&self) -> usize { self.row_stride_dim().value() }

	#[inline]
	fn row_stride_dim(&self) -> Self::RowStride;

	#[inline]
	fn col_stride(&self) -> usize { self.col_stride_dim().value() }

	#[inline]
	fn col_stride_dim(&self) -> Self::ColStride;

	#[inline]
	fn strides(&self) -> Strides<Self::RowStride, Self::ColStride> { Strides::new(self.row_stride_dim(), self.col_stride_dim()) }

	#[inline]
	fn row_index(&self, p: usize) -> usize { p * self.row_stride() }

	#[inline]
	fn col_index(&self, p: usize) -> usize { p * self.col_stride() }

	#[inline]
	fn index(&self, r: usize, c: usize) -> usize { r * self.row_stride() + c * self.col_stride() }

	#[inline]
	fn equal_strides<OS: Strided>(&self, o: &OS) -> bool {
		self.col_stride() == o.col_stride() && self.row_stride() == o.row_stride()
	}
}