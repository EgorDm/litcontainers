use super::Dim;

pub enum Axis {
	Row,
	Col,
}

// TODO: Can we write it more generic for possible 3d sizes?
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Size<R, C> {
	rows: R,
	cols: C,
}

impl<R: Dim, C: Dim> StorageSize<R, C> for Size<R, C> {
	fn rows_dim(&self) -> R { self.rows }

	fn cols_dim(&self) -> C { self.cols }
}

pub trait StorageSize<R: Dim, C: Dim> {
	fn rows(&self) -> usize { self.rows_dim().value() }

	fn rows_dim(&self) -> R;

	fn cols(&self) -> usize { self.cols_dim().value() }

	fn cols_dim(&self) -> C;

	fn size(&self) -> usize { self.rows() * self.cols() }
}

pub struct Strides<RS, CS> {
	row_stride: RS,
	col_stride: CS,
}

impl<RS: Dim, CS: Dim> StorageStrides<RS, CS> for Strides<RS, CS> {
	fn row_stride_dim(&self) -> RS { self.row_stride }

	fn col_stride_dim(&self) -> CS { self.col_stride }
}

pub trait StorageStrides<RS: Dim, CS: Dim> {
	fn row_stride(&self) -> usize { self.row_stride_dim().value() }

	fn row_stride_dim(&self) -> RS;

	fn col_stride(&self) -> usize { self.col_stride_dim().value() }

	fn col_stride_dim(&self) -> CS;
}