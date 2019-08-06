use crate::{format::*, storage::*, RowVec};
use num_traits::Signed;

pub fn find_nearest<T, R, C, S>(s: &S, v: T) -> usize
	where T: ElementaryScalar + Signed, R: Dim, C: Dim,  S: Storage<T, R, C>
{
	let mut nearest_diff = T::max_val();
	let mut nearest_idx = 0;

	for (i, x) in s.as_iter().enumerate() {
		let diff = (*x - v).abs();
		if diff < nearest_diff {
			nearest_diff = diff;
			nearest_idx = i;
		}
	}

	nearest_idx
}

// TODO: Add find nearest using binary search