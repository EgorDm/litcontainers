use crate::{format::*, storage::*, RowVec};

pub fn argmax_cols<T, S>(s: &S) -> RowVec<u32, S::Rows>
	where T: Scalar,  S: Storage<T>
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

pub fn max_bucket_cols<T, S, TA, SA>(s: &S, axis: &SA) -> RowVec<TA, S::Rows>
	where T: Scalar, S: Storage<T>,
	      TA: Scalar, SA: RowVecStorage<TA> + StorageSize<Cols=S::Cols>
{
	let maxi = argmax_cols(s);
	let mut ret = rvec_zeros![s.row_dim()];
	for (i, v) in maxi.as_iter().zip(ret.as_iter_mut()) {
		*v = axis[*i as usize];
	}
	ret
}