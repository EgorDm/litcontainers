use crate::storage::*;
use crate::format::{Dim, Scalar};

pub trait StorageConstructor<T, R, C>: StorageMut<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	#[inline]
	fn from_value(rows: R, cols: C, value: T) -> Self;

	#[inline]
	fn zeros(rows: R, cols: C) -> Self {
		Self::from_value(rows, cols, T::default())
	}

	fn linspace_rows(rows: R, cols: C, start: T, end: T) -> Self {
		let interval = (end - start) / T::from_usize(cols.value() - 1);
		let mut ret = Self::zeros(rows, cols);

		for r in 0..rows.value() {
			let mut agg = start;
			for s in ret.as_row_slice_mut_iter(r) {
				*s = agg;
				agg += interval;
			}
		}
		ret
	}

	fn linspace_cols(rows: R, cols: C, start: T, end: T) -> Self {
		let interval = (end - start) / T::from_usize(rows.value() - 1);
		let mut ret = Self::zeros(rows, cols);

		for r in 0..cols.value() {
			let mut agg = start;
			for s in ret.as_col_slice_mut_iter(r) {
				*s = agg;
				agg += interval;
			}
		}
		ret
	}

	fn regspace_rows(rows: R, cols: C, start: T) -> Self {
		Self::regspace_step_rows(rows, cols, start, T::from(1).unwrap())
	}

	fn regspace_step_rows(rows: R, cols: C, start: T, step: T) -> Self {
		let mut ret = Self::zeros(rows, cols);
		for c in 0..rows.value() {
			let mut agg = start;
			for s in ret.as_row_slice_mut_iter(c) {
				*s = agg;
				agg += step;
			}
		}
		ret
	}

	fn regspace_cols(rows: R, cols: C, start: T) -> Self {
		Self::regspace_step_cols(rows, cols, start, T::from(1).unwrap())
	}

	fn regspace_step_cols(rows: R, cols: C, start: T, step: T) -> Self {
		let mut ret = Self::zeros(rows, cols);
		for c in 0..cols.value() {
			let mut agg = start;
			for s in ret.as_col_slice_mut_iter(c) {
				*s = agg;
				agg += step;
			}
		}
		ret
	}
}