use crate::storage::*;
use crate::format::{Dim, Scalar};

pub trait StorageConstructor<T, R, C>: StorageMut<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	/// Creates a container with all elements set to given value.
	#[inline]
	fn from_value(rows: R, cols: C, value: T) -> Self;

	/// Creates a container with all elements set to `0`.
	#[inline]
	fn zeros(rows: R, cols: C) -> Self {
		Self::from_value(rows, cols, T::default())
	}

	// Crate a container from a vector containing the data. Data must be stored in row wise order.
	fn from_vec(rows: R, cols: C, data: Vec<T>) -> Self {
		assert_eq!(rows.value() * cols.value(), data.len());
		let mut ret = Self::zeros(rows, cols);
		for (o, i) in ret.as_iter_mut().zip(data) { *o = i; }
		ret
	}

	/// Creates a container with all rows containing regularly spaced values from start to end.
	fn linspace_rows(rows: R, cols: C, start: T, end: T) -> Self {
		let interval = (end - start) / T::from_usize(cols.value() - 1);
		let mut ret = Self::zeros(rows, cols);

		for r in 0..rows.value() {
			let mut agg = start;
			for s in ret.slice_rows_as_mut_iter(r) {
				*s = agg;
				agg += interval;
			}
		}
		ret
	}

	/// Creates a container with all cols containing regularly spaced values from start to end.
	fn linspace_cols(rows: R, cols: C, start: T, end: T) -> Self {
		let interval = (end - start) / T::from_usize(rows.value() - 1);
		let mut ret = Self::zeros(rows, cols);

		for r in 0..cols.value() {
			let mut agg = start;
			for s in ret.slice_cols_as_mut_iter(r) {
				*s = agg;
				agg += interval;
			}
		}
		ret
	}

	/// Creates a container with all rows containing regularly spaced values from start to start + col_count.
	fn regspace_rows(rows: R, cols: C, start: T) -> Self {
		Self::regspace_step_rows(rows, cols, start, T::from(1).unwrap())
	}

	/// Creates a container with all rows containing regularly spaced values from start to start + col_count * step.
	fn regspace_step_rows(rows: R, cols: C, start: T, step: T) -> Self {
		let mut ret = Self::zeros(rows, cols);
		for c in 0..rows.value() {
			let mut agg = start;
			for s in ret.slice_rows_as_mut_iter(c) {
				*s = agg;
				agg += step;
			}
		}
		ret
	}

	/// Creates a container with all cols containing regularly spaced values from start to start + row_count.
	fn regspace_cols(rows: R, cols: C, start: T) -> Self {
		Self::regspace_step_cols(rows, cols, start, T::from(1).unwrap())
	}

	/// Creates a container with all cols containing regularly spaced values from start to start + row_count * step.
	fn regspace_step_cols(rows: R, cols: C, start: T, step: T) -> Self {
		let mut ret = Self::zeros(rows, cols);
		for c in 0..cols.value() {
			let mut agg = start;
			for s in ret.slice_cols_as_mut_iter(c) {
				*s = agg;
				agg += step;
			}
		}
		ret
	}
}