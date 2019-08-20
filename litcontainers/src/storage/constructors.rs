use crate::storage::*;
use crate::format::*;
use rand::Rng;

pub trait StorageConstructor<T>: StorageMut<T>
	where T: Scalar
{
	/// Creates a container with all elements set to given value.
	#[inline]
	fn from_value(s: SSize<Self>, value: T) -> Self;

	/// Creates a container with all elements set to `0`.
	#[inline]
	fn zeros(s: SSize<Self>) -> Self {
		Self::from_value(s, T::default())
	}

	/// Creates a container with all elements set to a random value
	#[inline]
	fn rand(s: SSize<Self>) -> Self
		where rand::distributions::Standard: rand::distributions::Distribution<T>
	{
		let mut ret = Self::zeros(s);
		let mut rng = rand::thread_rng();
		for v in ret.as_iter_mut() { *v = rng.gen() }
		ret
	}

	/// Creates a container with all elements set to a random value
	#[inline]
	fn rand_range(s: SSize<Self>, from: T, to: T) -> Self
		where T: rand::distributions::uniform::SampleUniform
	{
		let mut ret = Self::zeros(s);
		let mut rng = rand::thread_rng();
		for v in ret.as_iter_mut() { *v = rng.gen_range(from, to) }
		ret
	}

	// Crate a container from a vector containing the data. Data must be stored in row wise order.
	fn from_vec(s: SSize<Self>, data: &[T]) -> Self {
		assert_eq!(s.len(), data.len());
		let mut ret = Self::zeros(s);
		for (o, i) in ret.as_iter_mut().zip(data) { *o = *i; }
		ret
	}

	/*
	/// Creates a container with all rows containing regularly spaced values from start to end.
	fn linspace_rows(s: SSize<Self>, start: T, end: T) -> Self {
		let interval = (end - start) / T::from_usize(s.cols() - 1);
		let mut ret = Self::zeros(s);

		for r in 0..s.rows() {
			let mut agg = start;
			for s in ret.slice_rows_as_mut_iter(r) {
				*s = agg;
				agg += interval;
			}
		}
		ret
	}

	/// Creates a container with all cols containing regularly spaced values from start to end.
	fn linspace_cols(s: SSize<Self>, start: T, end: T) -> Self {
		let interval = (end - start) / T::from_usize(rows.value() - 1);
		let mut ret = Self::zeros(s);

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
	fn regspace_rows(s: SSize<Self>, start: T) -> Self {
		Self::regspace_step_rows(s, start, T::from(1).unwrap())
	}

	/// Creates a container with all rows containing regularly spaced values from start to start + col_count * step.
	fn regspace_step_rows(s: SSize<Self>, start: T, step: T) -> Self {
		let mut ret = Self::zeros(s);
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
	fn regspace_cols(s: SSize<Self>, start: T) -> Self {
		Self::regspace_step_cols(s, start, T::from(1).unwrap())
	}

	/// Creates a container with all cols containing regularly spaced values from start to start + row_count * step.
	fn regspace_step_cols(s: SSize<Self>, start: T, step: T) -> Self {
		let mut ret = Self::zeros(s);
		for c in 0..cols.value() {
			let mut agg = start;
			for s in ret.slice_cols_as_mut_iter(c) {
				*s = agg;
				agg += step;
			}
		}
		ret
	}*/
}