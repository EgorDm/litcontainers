use crate::storage::*;
use crate::format::*;
use rand::Rng;

pub trait StorageConstructor<T>: StorageMut<T>
	where T: Element
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

	/// Creates a container with all rows containing regularly spaced values from start to end.
	fn linspace<A: Axis<Self::Rows, Self::Cols>>(s: SSize<Self>, _: A, start: T, end: T) -> Self
		where T: NumericElement
	{
		let interval = (end - start) / T::from_usize(s.cols() - 1);
		let mut ret = Self::zeros(s.clone());

		match A::axis_type() {
			AxisType::Row => {
				for r in 0..s.rows() {
					let mut agg = start;
					for s in ret.as_row_range_iter_mut(r) {
						*s = agg;
						agg += interval;
					}
				}
			},
			AxisType::Col => {
				for r in 0..s.cols() {
					let mut agg = start;
					for s in ret.as_col_range_iter_mut(r) {
						*s = agg;
						agg += interval;
					}
				}
			},
		}

		ret
	}

	/// Creates a container with all rows containing regularly spaced values from start to start + axis_size.
	fn regspace<A: Axis<Self::Rows, Self::Cols>>(s: SSize<Self>, axis: A, start: T) -> Self
		where T: NumericElement

	{
		Self::regspace_step(s, axis, start, T::from(1).unwrap())
	}

	/// Creates a container with all rows containing regularly spaced values from start to start + axis_size * step.
	fn regspace_step<A: Axis<Self::Rows, Self::Cols>>(s: SSize<Self>, _: A, start: T, step: T) -> Self
		where T: NumericElement
	{
		let mut ret = Self::zeros(s.clone());
		match A::axis_type() {
			AxisType::Row => {
				for c in 0..s.rows() {
					let mut agg = start;
					for s in ret.as_row_range_iter_mut(c) {
						*s = agg;
						agg += step;
					}
				}
			},
			AxisType::Col => {
				for c in 0..s.cols() {
					let mut agg = start;
					for s in ret.as_col_range_iter_mut(c) {
						*s = agg;
						agg += step;
					}
				}
			},
		}
		ret
	}
}