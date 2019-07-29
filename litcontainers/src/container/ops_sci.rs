use crate::format::*;
use crate::storage::*;
use crate::ops::*;
use super::container::*;
use num_traits::{Float, Signed};
use num_complex::Complex;

macro_rules! impl_unary_float_op (
	($GroupTrait: ident, $OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		impl<T, R, C, S> $OpTrait for Container<T, R, C, S>
			where
				T: Scalar + $GroupTrait, R: Dim, C: Dim, S: StorageMut<T, R, C>
		{
			type Output = <Self as Ownable<T, R, C>>::OwnedType;

			fn $op_fn(self) -> Self::Output {
				let mut ret = self.owned();
				ret.mapv_inplace(move |v| v.$op_fn());
				ret
			}
		}

		impl<T, R, C, S> $OpTrait for &Container<T, R, C, S>
			where
				T: Scalar + $GroupTrait, R: Dim, C: Dim, S: StorageMut<T, R, C>,
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self) -> Self::Output {
				let mut ret = self.clone_owned();
				ret.mapv_inplace(move |v| v.$op_fn());
				ret
			}
		}

		impl<T, R, C, S> $OpAssignTrait for Container<T, R, C, S>
			where
				T: Scalar + $GroupTrait, R: Dim, C: Dim, S: StorageMut<T, R, C>
		{
			fn $op_assign_fn(&mut self) {
				self.mapv_inplace(move |v| v.$op_fn());
			}
		}
	}
);

impl_unary_float_op!(Float, ASin, asin, ASinAssign, asin_assign);
impl_unary_float_op!(Float, Sin, sin, SinAssign, sin_assign);
impl_unary_float_op!(Float, ACos, acos, ACosAssign, acos_assign);
impl_unary_float_op!(Float, Cos, cos, CosAssign, cos_assign);
impl_unary_float_op!(Float, Tan, tan, TanAssign, tan_assign);
impl_unary_float_op!(Float, ATan, atan, ATanAssign, atan_assign);
impl_unary_float_op!(Float, Exp, exp, ExpAssign, exp_assign);
impl_unary_float_op!(Float, Ceil, ceil, CeilAssign, ceil_assign);
impl_unary_float_op!(Float, Floor, floor, FloorAssign, floor_assign);
impl_unary_float_op!(Float, Round, round, RoundAssign, round_assign);
impl_unary_float_op!(Signed, Abs, abs, AbsAssign, abs_assign);
impl_unary_float_op!(Float, Sqrt, sqrt, SqrtAssign, sqrt_assign);
impl_unary_float_op!(Float, Log2, log2, Log2Assign, log2_assign);
impl_unary_float_op!(Float, Log10, log10, Log10Assign, log10_assign);
impl_unary_float_op!(Float, Ln, ln, LnAssign, ln_assign);

impl<T, R, C, S, RT> Pow<RT> for Container<T, R, C, S>
	where
		T: Scalar + num_traits::Pow<RT, Output=T>, R: Dim, C: Dim, S: StorageMut<T, R, C>, RT: Scalar
{
	type Output = <Self as Ownable<T, R, C>>::OwnedType;

	fn pow(self, rhs: RT) -> Self::Output {
		let mut ret = self.owned();
		ret.mapv_inplace(move |v| num_traits::Pow::pow(v, rhs));
		ret
	}
}

impl<T, R, C, S, RT> Pow<RT> for &Container<T, R, C, S>
	where
		T: Scalar + num_traits::Pow<RT, Output=T>, R: Dim, C: Dim, S: StorageMut<T, R, C>, RT: Scalar
{
	type Output = Container<T, R, C, S::OwnedType>;

	fn pow(self, rhs: RT) -> Self::Output {
		let mut ret = self.clone_owned();
		ret.mapv_inplace(move |v| num_traits::Pow::pow(v, rhs));
		ret
	}
}

impl<T, R, C, S, RT> PowAssign<RT> for Container<T, R, C, S>
	where
		T: Scalar + num_traits::Pow<RT, Output=T>, R: Dim, C: Dim, S: StorageMut<T, R, C>, RT: Scalar
{
	fn pow_assign(&mut self, rhs: RT) {
		self.mapv_inplace(move |v| num_traits::Pow::pow(v, rhs));
	}
}

macro_rules! impl_binary_float_op (
	($GroupTrait: ident, $OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		impl<T, R, C, S> $OpTrait<T> for Container<T, R, C, S>
			where
				T: Scalar + $GroupTrait, R: Dim, C: Dim, S: StorageMut<T, R, C>
		{
			type Output = <Self as Ownable<T, R, C>>::OwnedType;

			fn $op_fn(self, rhs: T) -> Self::Output {
				let mut ret = self.owned();
				ret.mapv_inplace(move |v| v.$op_fn(rhs));
				ret
			}
		}

		impl<T, R, C, S> $OpTrait<T> for &Container<T, R, C, S>
			where
				T: Scalar + $GroupTrait, R: Dim, C: Dim, S: StorageMut<T, R, C>,
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: T) -> Self::Output {
				let mut ret = self.clone_owned();
				ret.mapv_inplace(move |v| v.$op_fn(rhs));
				ret
			}
		}

		impl<T, R, C, S> $OpAssignTrait<T> for Container<T, R, C, S>
			where
				T: Scalar + $GroupTrait, R: Dim, C: Dim, S: StorageMut<T, R, C>
		{
			fn $op_assign_fn(&mut self, rhs: T) {
				self.mapv_inplace(move |v| v.$op_fn(rhs));
			}
		}
	}
);

impl_binary_float_op!(Float, Log, log, LogAssign, log_assign);
impl_binary_float_op!(Float, Max, max, MaxAssign, max_assign);
impl_binary_float_op!(Float, Min, min, MinAssign, min_assign);

impl<T, R, C, S> Norm for &Container<Complex<T>, R, C, S>
	where T: ElementaryScalar + Float, R: Dim, C: Dim, S: StorageMut<Complex<T>, R, C>
{
	type Output = ContainerCM<T, R, C>;

	fn norm(self) -> Self::Output {
		let mut ret = ContainerCM::zeros(self.row_dim(), self.col_dim());
		for (o, s) in ret.as_row_mut_iter().zip(self.as_row_iter()) { *o = s.norm(); }
		ret
	}
}

impl<T, R, C, S> Sum for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	type Output = T;

	fn sum(&self) -> Self::Output {
		let mut ret = T::default();
		for v in self.as_row_iter() { ret += *v }
		ret
	}
}

impl<T, R, C, S> Mean for Container<T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	type Output = T;

	fn mean(&self) -> Self::Output {
		self.sum() / num_traits::cast(self.size()).unwrap()
	}
}

impl<T, R, C, S> Maximum for Container<T, R, C, S>
	where T: ElementaryScalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	type Output = T;

	fn maximum(&self) -> Self::Output {
		let mut ret = T::min_val();
		for v in self.as_iter() {
			if ret < *v { ret = *v; }
		}
		ret
	}
}

impl<T, R, C, S> Minimum for Container<T, R, C, S>
	where T: ElementaryScalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	type Output = T;

	fn minimum(&self) -> Self::Output {
		let mut ret = T::max_val();
		for v in self.as_iter() {
			if ret > *v { ret = *v; }
		}
		ret
	}
}

// Clamp
impl<T, R, C, S> Clamp<T> for Container<T, R, C, S>
	where
		T: Scalar + Float, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	type Output = <Self as Ownable<T, R, C>>::OwnedType;

	fn clamp(self, min: T, max: T) -> Self::Output {
		let mut ret = self.owned();
		ret.mapv_inplace(move |x| {
			if x < min { min }
			else if x > max { max }
			else { x }
		});
		ret
	}
}
impl<T, R, C, S> Clamp<T> for &Container<T, R, C, S>
	where
		T: Scalar + Float, R: Dim, C: Dim, S: StorageMut<T, R, C>,
{
	type Output = Container<T, R, C, S::OwnedType>;

	fn clamp(self, min: T, max: T) -> Self::Output {
		let mut ret = self.clone_owned();
		ret.mapv_inplace(move |x| {
			if x < min { min }
			else if x > max { max }
			else { x }
		});
		ret
	}
}
impl<T, R, C, S> ClampAssign<T> for Container<T, R, C, S>
	where
		T: Scalar + Float, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	fn clamp_assign(&mut self, min: T, max: T) {
		self.mapv_inplace(move |x| {
			if x < min { min }
			else if x > max { max }
			else { x }
		});
	}
}