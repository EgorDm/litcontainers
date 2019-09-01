use crate::*;

#[derive(new)]
pub struct Sum<L>
	where L: Operation
{
	left: L
}

impl<L> Operation for Sum<L>
	where L: Operation, L::Result: InplaceForeach<L::Type>,
	      L::Type: NumericElement
{
	type Type = L::Type;
	type Rows = U1;
	type Cols = U1;
	type Result = L::Type;

	fn apply(self) -> Self::Result {
		let mut ret = L::Type::default();
		self.left.apply().foreach(|v| ret += *v);
		ret
	}
}

pub fn cumsum<T, S, A>(s: &S, _: A) -> ContainerRM<T, S::Rows, S::Cols>
	where T: NumericElement, S: Storage<T>, A: Axis<S::Rows, S::Cols>
{
	let mut ret = ContainerRM::zeros(s.size());
	match A::axis_type() {
		AxisType::Row => {
			for (row_in, mut row_out) in s.as_row_slice_iter().zip(ret.as_row_slice_iter_mut()) {
				let mut acc = T::default();
				for (v_in, v_out) in row_in.as_row_iter().zip(row_out.as_row_iter_mut()) {
					acc += *v_in;
					*v_out = acc;
				}
			}
		},
		AxisType::Col => {
			for (col_in, mut col_out) in s.as_col_slice_iter().zip(ret.as_col_slice_iter_mut()) {
				let mut acc = T::default();
				for (v_in, v_out) in col_in.as_col_iter().zip(col_out.as_col_iter_mut()) {
					acc += *v_in;
					*v_out = acc;
				}
			}
		},
	}
	ret
}

pub fn sum_rows<T, S>(s: &S) -> ColVec<T, S::Rows>
	where T: NumericElement, S: Storage<T>
{
	let mut ret = cvec_zeros![s.row_dim()];
	for (row_in, out) in s.as_row_slice_iter().zip(ret.as_row_iter_mut()) {
		for v_in in row_in.as_row_iter() {
			*out += *v_in;
		}
	}
	ret
}

pub fn sum_cols<T, S>(s: &S) -> RowVec<T, S::Cols>
	where T: NumericElement, S: Storage<T>
{
	let mut ret = rvec_zeros![s.col_dim()];
	for (col_in, out) in s.as_col_slice_iter().zip(ret.as_col_iter_mut()) {
		for v_in in col_in.as_col_iter() {
			*out += *v_in;
		}
	}
	ret
}

// TODO: simplify using sum and map divide

pub fn mean_rows<T, S>(s: &S) -> ColVec<T, S::Rows>
	where T: NumericElement, S: Storage<T>
{
	let mut ret = cvec_zeros![s.row_dim()];
	let elem_count = num_traits::cast(s.cols()).unwrap();
	for (row_in, out) in s.as_row_slice_iter().zip(ret.as_row_iter_mut()) {
		for v_in in row_in.as_row_iter() {
			*out += *v_in;
		}
		*out /= elem_count;
	}
	ret
}

pub fn mean_cols<T, S>(s: &S) -> RowVec<T, S::Cols>
	where T: NumericElement, S: Storage<T>
{
	let mut ret = rvec_zeros![s.col_dim()];
	let elem_count = num_traits::cast(s.rows()).unwrap();
	for (col_in, out) in s.as_col_slice_iter().zip(ret.as_col_iter_mut()) {
		for v_in in col_in.as_col_iter() {
			*out += *v_in;
		}
		*out /= elem_count;
	}
	ret
}

pub trait SumOperations<T: NumericElement>: Storage<T> {
	fn sum(&self) -> T { Sum::new(BorrowedProvider::new(self)).apply() }

	fn mean(&self) -> T { Sum::new(BorrowedProvider::new(self)).apply() / num_traits::cast(self.len()).unwrap() }

	fn sum_rows(&self) -> ColVec<T, Self::Rows> { sum_rows(self) }

	fn sum_cols(&self) -> RowVec<T, Self::Cols> { sum_cols(self) }

	fn mean_rows(&self) -> ColVec<T, Self::Rows> { mean_rows(self) }

	fn mean_cols(&self) -> RowVec<T, Self::Cols> { mean_cols(self) }

	fn cumsum<A: Axis<Self::Rows, Self::Cols>>(&self, a: A) -> ContainerRM<T, Self::Rows, Self::Cols> { cumsum(self, a) }
}

impl<T: NumericElement, S: Storage<T>> SumOperations<T> for S {}