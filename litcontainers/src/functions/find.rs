use crate::{format::*, storage::*};
use num_traits::Signed;

pub fn find_nearest<T, S>(s: &S, v: T) -> usize
	where T: ElementaryScalar + Signed, S: Storage<T>
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