/*
use crate::{format::*, storage::*, RowVec};

pub fn argmax_cols<T, R, C, S>(s: &S) -> RowVec<u32, R>
	where T: ElementaryScalar, R: Dim, C: Dim,  S: Storage<T, R, C>
{
	let mut ret = rvec_zeros![s.row_dim()];
	for (col_in, out) in s.as_col_slice_iter().zip(ret.as_iter_mut()) {
		let mut max_val = T::min_val();
		for (i, v_in) in col_in.as_col_iter().enumerate() {
			if *v_in > max_val {
				max_val = *v_in;
				*out = i as u32;
			}
		}
	}
	ret
}

pub fn max_bucket_cols<T, R, C, S, TA, SA>(s: &S, axis: &SA) -> RowVec<TA, R>
	where T: ElementaryScalar, R: Dim, C: Dim,  S: Storage<T, R, C>,
	      TA: ElementaryScalar, SA: RowVecStorage<TA, C>
{
	let maxi = argmax_cols(s);
	let mut ret = rvec_zeros![s.row_dim()];
	for (i, v) in maxi.as_iter().zip(ret.as_iter_mut()) {
		*v = axis[*i as usize];
	}
	ret
}*/
