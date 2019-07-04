use crate::format::*;
use crate::storage::*;
use crate::ops::*;
use super::slice::*;
use crate::container::Container;
use num_traits::Float;

macro_rules! impl_unary_float_op (
	($OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		impl<'a, T, R, C, S> $OpTrait for &SliceBase<'a, T, R, C, S>
			where
				T: Scalar + Float, R: Dim, C: Dim, S: Storage<T, R, C>,
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self) -> Self::Output {
				let mut ret = self.clone_owned();
				for o in ret.as_row_mut_iter() {
					*o = o.$op_fn();
				}
				ret
			}
		}

		impl<'a, T, R, C, S> $OpAssignTrait for SliceBase<'a, T, R, C, S>
			where
				T: Scalar + Float, R: Dim, C: Dim, S: StorageMut<T, R, C>
		{
			fn $op_assign_fn(&mut self) {
				for o in self.as_row_mut_iter() {
					*o = o.$op_fn();
				}
			}
		}
	}
);

impl_unary_float_op!(ASin, asin, ASinAssign, asin_assign);
impl_unary_float_op!(Sin, sin, SinAssign, sin_assign);
impl_unary_float_op!(ACos, acos, ACosAssign, acos_assign);
impl_unary_float_op!(Cos, cos, CosAssign, cos_assign);
impl_unary_float_op!(Tan, tan, TanAssign, tan_assign);
impl_unary_float_op!(ATan, atan, ATanAssign, atan_assign);
impl_unary_float_op!(Exp, exp, ExpAssign, exp_assign);
impl_unary_float_op!(Ceil, ceil, CeilAssign, ceil_assign);
impl_unary_float_op!(Floor, floor, FloorAssign, floor_assign);
impl_unary_float_op!(Round, round, RoundAssign, round_assign);
impl_unary_float_op!(Abs, abs, AbsAssign, abs_assign);
impl_unary_float_op!(Sqrt, sqrt, SqrtAssign, sqrt_assign);
impl_unary_float_op!(Log2, log2, Log2Assign, log2_assign);
impl_unary_float_op!(Log10, log10, Log10Assign, log10_assign);
impl_unary_float_op!(Ln, ln, LnAssign, ln_assign);


impl<'a, T, R, C, S, RT> Pow<RT> for &SliceBase<'a, T, R, C, S>
	where
		T: Scalar + num_traits::Pow<RT, Output=T>, R: Dim, C: Dim, S: Storage<T, R, C>, RT: Scalar
{
	type Output = Container<T, R, C, S::OwnedType>;

	fn pow(self, rhs: RT) -> Self::Output {
		let mut ret = self.clone_owned();
		for o in ret.as_row_mut_iter() {
			*o = num_traits::Pow::pow(*o, rhs);
		}
		ret
	}
}

impl<'a, T, R, C, S, RT> PowAssign<RT> for SliceBase<'a, T, R, C, S>
	where
		T: Scalar + num_traits::Pow<RT, Output=T>, R: Dim, C: Dim, S: StorageMut<T, R, C>, RT: Scalar
{
	fn pow_assign(&mut self, rhs: RT) {
		for o in self.as_row_mut_iter() {
			*o = num_traits::Pow::pow(*o, rhs);
		}
	}
}

macro_rules! impl_binary_float_op (
	($OpTrait: ident, $op_fn: ident, $OpAssignTrait: ident, $op_assign_fn: ident) => {
		impl<'a, T, R, C, S> $OpTrait<T> for &SliceBase<'a, T, R, C, S>
			where
				T: Scalar + Float, R: Dim, C: Dim, S: Storage<T, R, C>,
		{
			type Output = Container<T, R, C, S::OwnedType>;

			fn $op_fn(self, rhs: T) -> Self::Output {
				let mut ret = self.clone_owned();
				for o in ret.as_row_mut_iter() {
					*o = o.$op_fn(rhs);
				}
				ret
			}
		}

		impl<'a, T, R, C, S> $OpAssignTrait<T> for SliceBase<'a, T, R, C, S>
			where
				T: Scalar + Float, R: Dim, C: Dim, S: StorageMut<T, R, C>
		{
			fn $op_assign_fn(&mut self, rhs: T) {
				for o in self.as_row_mut_iter() {
					*o = o.$op_fn(rhs);
				}
			}
		}
	}
);

impl_binary_float_op!(Log, log, LogAssign, log_assign);
impl_binary_float_op!(Max, max, MaxAssign, max_assign);
impl_binary_float_op!(Min, min, MinAssign, min_assign);
