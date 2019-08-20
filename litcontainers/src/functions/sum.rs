/*
use crate::*;

pub fn cumsum_rows<T, R, C, S>(s: &S) -> ContainerRM<T, R, C>
	where T: ElementaryScalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	let mut ret = ContainerRM::zeros(s.row_dim(), s.col_dim());
	for (row_in, mut row_out) in s.as_row_slice_iter().zip(ret.as_row_slice_mut_iter()) {
		let mut acc = T::default();
		for (v_in, v_out) in row_in.as_row_iter().zip(row_out.as_row_mut_iter()) {
			acc += *v_in;
			*v_out = acc;
		}
	}
	ret
}

pub fn cumsum_cols<T, R, C, S>(s: &S) -> ContainerCM<T, R, C>
	where T: ElementaryScalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	let mut ret = ContainerCM::zeros(s.row_dim(), s.col_dim());
	for (col_in, mut col_out) in s.as_col_slice_iter().zip(ret.as_col_slice_mut_iter()) {
		let mut acc = T::default();
		for (v_in, v_out) in col_in.as_col_iter().zip(col_out.as_col_mut_iter()) {
			acc += *v_in;
			*v_out = acc;
		}
	}
	ret
}

pub fn sum_rows<T, R, C, S>(s: &S) -> ColVec<T, R>
	where T: ElementaryScalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	let mut ret = cvec_zeros![s.row_dim()];
	for (row_in, out) in s.as_row_slice_iter().zip(ret.as_row_mut_iter()) {
		for v_in in row_in.as_row_iter() {
			*out += *v_in;
		}
	}
	ret
}

pub fn sum_cols<T, R, C, S>(s: &S) -> RowVec<T, C>
	where T: ElementaryScalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	let mut ret = rvec_zeros![s.col_dim()];
	for (col_in, out) in s.as_col_slice_iter().zip(ret.as_col_mut_iter()) {
		for v_in in col_in.as_col_iter() {
			*out += *v_in;
		}
	}
	ret
}

// TODO: simplify using sum and map divide

pub fn mean_rows<T, R, C, S>(s: &S) -> ColVec<T, R>
	where T: ElementaryScalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	let mut ret = cvec_zeros![s.row_dim()];
	let elem_count = num_traits::cast(s.col_count()).unwrap();
	for (row_in, out) in s.as_row_slice_iter().zip(ret.as_row_mut_iter()) {
		for v_in in row_in.as_row_iter() {
			*out += *v_in;
		}
		*out /= elem_count;
	}
	ret
}

pub fn mean_cols<T, R, C, S>(s: &S) -> RowVec<T, C>
	where T: ElementaryScalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	let mut ret = rvec_zeros![s.col_dim()];
	let elem_count = num_traits::cast(s.row_count()).unwrap();
	for (col_in, out) in s.as_col_slice_iter().zip(ret.as_col_mut_iter()) {
		for v_in in col_in.as_col_iter() {
			*out += *v_in;
		}
		*out /= elem_count;
	}
	ret
}*/
